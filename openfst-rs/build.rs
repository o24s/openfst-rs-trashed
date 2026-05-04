use ignore::Walk;
use regex::Regex;
use std::{
    collections::HashMap,
    env,
    error::Error,
    path::{Path, PathBuf},
};

include!("src/generated_srcs.rs");

fn get_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<_> = Walk::new(dir)
        .flatten()
        .filter(|e| !e.path().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect();
    files.sort();
    files
}

fn configure_compiler(build: &mut cc::Build) {
    build
        .cpp(true)
        .debug(false)
        // .opt_level(2)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-w")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-sign-compare");
}

fn is_clippy_or_check() -> bool {
    let is_clippy = env::var("RUSTC_WORKSPACE_WRAPPER")
        .map(|w| w.contains("clippy"))
        .unwrap_or(false)
        || env::var("RUSTC_WRAPPER")
            .map(|w| w.contains("clippy"))
            .unwrap_or(false);

    let skip_env = env::var("SKIP_CXX_COMPILE").is_ok();

    is_clippy || skip_env
}

fn build_openfst(manifest_dir: &Path, skip_compile: bool) -> Result<(), Box<dyn Error>> {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let use_cache = env::var_os("CARGO_FEATURE_CACHE_DEPS").is_some();
    let cache_dir = env::var_os("OPENFST_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join(".build-cache"));

    let lib_name = "libopenfst.a";
    let cached_lib = cache_dir.join(lib_name);

    if use_cache && cached_lib.exists() {
        println!("cargo:rustc-link-search=native={}", cache_dir.display());
        println!("cargo:rustc-link-lib=static=openfst");
        return Ok(());
    }

    if skip_compile {
        println!("cargo:rustc-link-lib=static=openfst");
        return Ok(());
    }

    let vendor_dir = manifest_dir.join("vendor");
    let mut build = cc::Build::new();

    configure_compiler(&mut build);

    build
        .include(vendor_dir.join("openfst"))
        .include(vendor_dir.join("abseil-cpp"));

    if target_os == "windows" || target_os == "ios" {
        build.define("FST_NO_DYNAMIC_LINKING", None);
    }

    for src in OPENFST_SRCS {
        let file_name = Path::new(src).file_name().unwrap().to_string_lossy();

        let is_win = file_name.contains("_win.") || file_name.contains("_windows.");
        let is_posix = file_name.contains("_posix.");
        let is_mac = file_name.contains("_mac.")
            || file_name.contains("_macos.")
            || file_name.contains("_darwin.")
            || file_name.contains("_apple.");

        if (is_win && target_os != "windows")
            || (is_posix && target_family != "unix")
            || (is_mac && target_os != "macos" && target_os != "ios")
        {
            continue;
        }
        build.file(src);
    }

    build.compile("openfst");

    if use_cache {
        std::fs::create_dir_all(&cache_dir)?;
        let out_dir = PathBuf::from(env::var("OUT_DIR")?);
        let compiled_lib = out_dir.join(lib_name);
        if compiled_lib.exists() {
            std::fs::copy(&compiled_lib, &cached_lib)?;
        }
        println!("cargo:rustc-link-search=native={}", cache_dir.display());
    }

    println!("cargo:rustc-link-lib=static=openfst");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let manifest_dir = Path::new(&manifest_dir);
    let out_dir = env::var("OUT_DIR")?;
    let out_dir_path = Path::new(&out_dir);

    let skip_compile = is_clippy_or_check();

    build_openfst(manifest_dir, skip_compile)?;

    let ffi_rs_content = include_str!("src/ffi.rs");

    let arc_macros = parse_arc_macros(ffi_rs_content, out_dir_path);

    let bridge_files = get_files(&manifest_dir.join("src/ffi"));
    let mut cxx_input_files = Vec::new();

    let cxx_common = include_str!("src/ffi/cxx_common.rs.decl");

    for file in bridge_files {
        if file.extension().is_some_and(|ext| ext == "decl") {
            continue;
        }

        let content = std::fs::read_to_string(&file)?;

        if let Some((template_name, mod_decl, inner_body)) = extract_template_parts(&content) {
            let expanded_body = expand_template(&inner_body, &arc_macros);

            let expanded_with_common = expanded_body.replacen(
                "unsafe extern \"C++\" {",
                &format!("unsafe extern \"C++\" {{\n{}", cxx_common),
                1,
            );

            let final_code = format!(
                "#[cxx::bridge(namespace = \"fst_rust::ffi\")]\n{} {{\n{}}}",
                mod_decl, expanded_with_common
            );

            let out_file_name = format!("bridge_{}.rs", template_name);
            let out_path = out_dir_path.join(&out_file_name);
            std::fs::write(&out_path, final_code)?;

            cxx_input_files.push(out_path);
        } else {
            let rel_path = file
                .strip_prefix(manifest_dir)
                .unwrap_or(&file)
                .to_path_buf();
            cxx_input_files.push(rel_path);
        }
    }

    let cpp_files = [
        "cpp/wrapper.cpp",
        "cpp/symbol-table.cc",
        "cpp/sparse-power-weight.cc",
    ];

    let mut bridge_build = cxx_build::bridges(&cxx_input_files);

    configure_compiler(&mut bridge_build);

    bridge_build
        .files(cpp_files)
        .include("vendor/openfst")
        .include("vendor/abseil-cpp")
        .include(".")
        .include("cpp")
        .include(out_dir_path);

    if skip_compile {
        println!("cargo:warning=Clippy mode: Skipping bridge C++ compilation");
    } else {
        bridge_build.compile("openfst_shim");
    }

    for watch_file in [
        "vendor/openfst",
        "vendor/abseil-cpp",
        "cpp",
        "src/ffi",
        "src/generated_srcs.rs",
        "build.rs",
        "src/ffi/cxx_common.rs.inc",
    ] {
        println!("cargo:rerun-if-changed={}", watch_file);
    }

    println!("cargo:rerun-if-env-changed=OPENFST_CACHE_DIR");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_CACHE_DEPS");
    println!("cargo:rerun-if-env-changed=SKIP_CXX_COMPILE");

    Ok(())
}

fn parse_arc_macros(content: &str, out_dir: &Path) -> HashMap<String, Vec<Vec<(String, String)>>> {
    let mut map = HashMap::new();
    let re_macro = Regex::new(r"macro_rules!\s+(\w+)\s*\{([\s\S]*?)\};").unwrap();
    let re_line = Regex::new(r"\$(?:crate::)?\w+!\(([^;]+)\);").unwrap();

    let mut arc_with_semiring = Vec::new();
    let mut path_arcs = Vec::new();
    let mut convertible_semirings = Vec::new();

    for cap in re_macro.captures_iter(content) {
        let macro_name = cap[1].to_string();
        let body = &cap[2];

        let mut expansions = Vec::new();
        for line_cap in re_line.captures_iter(body) {
            let args = line_cap[1].to_string();

            let clean_args = args.replace("$(", "").replace(")*", "");
            let parts: Vec<String> = clean_args
                .split(',')
                .map(|s| {
                    let trimmed = s.trim();
                    // extract only last element (e.g., crate::ffi::weight::PairWeightValueF32 -> PairWeightValueF32)
                    if let Some(last_part) = trimmed.split("::").last() {
                        last_part.to_string()
                    } else {
                        trimmed.to_string()
                    }
                })
                .filter(|s| !s.is_empty() && !s.starts_with('$'))
                .collect();

            if parts.is_empty() {
                continue;
            }

            if parts[0].contains("cartesian_inner") || parts[0].contains("cartesian_apply") {
                continue;
            }

            if macro_name == "for_each_arc_with_semiring" && parts.len() >= 4 {
                arc_with_semiring.push((
                    parts[0].clone(), // arc
                    parts[1].clone(), // wtype
                    parts[2].clone(), // sfx
                    parts[3].clone(), // semiring
                ));
            } else if macro_name == "for_each_path_arc_type" && parts.len() >= 3 {
                path_arcs.push((
                    parts[0].clone(), // arc
                    parts[1].clone(), // wtype
                    parts[2].clone(), // sfx
                ));
            } else if macro_name == "convertible_semiring_rules" && parts.len() >= 2 {
                convertible_semirings.push((
                    parts[0].clone(), // semiring1
                    parts[1].clone(), // semiring2
                ));
            }

            let mut replacements = Vec::new();

            if macro_name.ends_with("_pair") && parts.len() >= 4 {
                replacements.push(("__SFX1__".to_string(), parts[2].clone()));
                replacements.push(("__SFX2__".to_string(), parts[3].clone()));
            } else if parts.len() >= 4 {
                replacements.push(("__WTYPE__".to_string(), parts[1].clone()));
                replacements.push(("__SFX__".to_string(), parts[2].clone()));
                replacements.push(("__SEMIRING__".to_string(), parts[3].clone()));
            } else if parts.len() >= 3 {
                replacements.push(("__WTYPE__".to_string(), parts[1].clone()));
                replacements.push(("__SFX__".to_string(), parts[2].clone()));
            } else if !parts.is_empty() {
                replacements.push(("__SFX__".to_string(), parts.last().unwrap().clone()));
            }

            if !replacements.is_empty() {
                expansions.push(replacements);
            }
        }
        if !expansions.is_empty() {
            map.insert(macro_name.clone(), expansions);
        }
    }

    if !arc_with_semiring.is_empty() {
        let mut for_each_arc_type = Vec::new();
        for (_, wtype, sfx, _) in &arc_with_semiring {
            for_each_arc_type.push(vec![
                ("__WTYPE__".to_string(), wtype.clone()),
                ("__SFX__".to_string(), sfx.clone()),
            ]);
        }
        map.insert("for_each_arc_type".to_string(), for_each_arc_type.clone());

        let mut pair_expansions = Vec::new();
        for exp1 in &for_each_arc_type {
            for exp2 in &for_each_arc_type {
                let sfx1 = exp1.iter().find(|(k, _)| k == "__SFX__").unwrap().1.clone();
                let sfx2 = exp2.iter().find(|(k, _)| k == "__SFX__").unwrap().1.clone();
                pair_expansions.push(vec![
                    ("__SFX1__".to_string(), sfx1),
                    ("__SFX2__".to_string(), sfx2),
                ]);
            }
        }
        map.insert("for_each_arc_type_pair".to_string(), pair_expansions);

        if !convertible_semirings.is_empty() {
            let mut convertible_pair_expansions = Vec::new();
            for (sr1, sr2) in &convertible_semirings {
                for (_, _, sfx1, semiring1) in &arc_with_semiring {
                    for (_, _, sfx2, semiring2) in &arc_with_semiring {
                        if semiring1 == sr1 && semiring2 == sr2 {
                            convertible_pair_expansions.push(vec![
                                ("__SFX1__".to_string(), sfx1.clone()),
                                ("__SFX2__".to_string(), sfx2.clone()),
                            ]);
                        }
                    }
                }
            }
            map.insert(
                "for_each_convertible_arc_pair".to_string(),
                convertible_pair_expansions,
            );
        }

        let mut cpp = String::new();
        cpp.push_str("// AUTOGENERATED BY build.rs - DO NOT EDIT\n");
        cpp.push_str("#pragma once\n\n");

        cpp.push_str("#define FOR_EACH_ARC_WITH_SEMIRING(M, ...) \\\n");
        for (arc_rs, wtype_rs, sfx, semiring) in &arc_with_semiring {
            let base_arc = arc_rs.split("::").last().unwrap();
            let arc_cpp = format!("fst::{}", base_arc);

            let wtype_cpp = match wtype_rs.split("::").last().unwrap() {
                "PairWeightValueF32" => "fst_rust::ffi::PairWeightValueF32",
                "PairWeightValueF64" => "fst_rust::ffi::PairWeightValueF64",
                "LexicographicWeightValueF32" => "fst_rust::ffi::LexicographicWeightValueF32",
                "LexicographicWeightValueF64" => "fst_rust::ffi::LexicographicWeightValueF64",
                "ExpectationWeightValueF32" => "fst_rust::ffi::ExpectationWeightValueF32",
                "ExpectationWeightValueF64" => "fst_rust::ffi::ExpectationWeightValueF64",
                "ExpectationWeightValuePairF32" => "fst_rust::ffi::ExpectationWeightValuePairF32",
                "ExpectationWeightValuePairF64" => "fst_rust::ffi::ExpectationWeightValuePairF64",
                "f32" => "float",
                "f64" => "double",
                other => other,
            };

            cpp.push_str(&format!(
                "    M({}, {}, {}, {}, ##__VA_ARGS__) \\\n",
                sfx,
                arc_cpp,
                wtype_cpp,
                semiring.to_uppercase()
            ));
        }
        cpp.push('\n');

        if !path_arcs.is_empty() {
            cpp.push_str("#define FOR_EACH_PATH_ARC_TYPE(M, ...) \\\n");
            for (arc_rs, wtype_rs, sfx) in &path_arcs {
                let base_arc = arc_rs.split("::").last().unwrap();
                let arc_cpp = format!("fst::{}", base_arc);

                let wtype_cpp = match wtype_rs.split("::").last().unwrap() {
                    "PairWeightValueF32" => "fst_rust::ffi::PairWeightValueF32",
                    "PairWeightValueF64" => "fst_rust::ffi::PairWeightValueF64",
                    "LexicographicWeightValueF32" => "fst_rust::ffi::LexicographicWeightValueF32",
                    "LexicographicWeightValueF64" => "fst_rust::ffi::LexicographicWeightValueF64",
                    "ExpectationWeightValueF32" => "fst_rust::ffi::ExpectationWeightValueF32",
                    "ExpectationWeightValueF64" => "fst_rust::ffi::ExpectationWeightValueF64",
                    "ExpectationWeightValuePairF32" => {
                        "fst_rust::ffi::ExpectationWeightValuePairF32"
                    }
                    "ExpectationWeightValuePairF64" => {
                        "fst_rust::ffi::ExpectationWeightValuePairF64"
                    }
                    "f32" => "float",
                    "f64" => "double",
                    other => other,
                };

                cpp.push_str(&format!(
                    "    M({}, {}, {}, ##__VA_ARGS__) \\\n",
                    sfx, arc_cpp, wtype_cpp
                ));
            }
            cpp.push('\n');
        }

        cpp.push_str("#define FOR_EACH_CONVERTIBLE_ARC_PAIR(M) \\\n");
        for (sr1, sr2) in &convertible_semirings {
            for (arc_rs1, _, sfx1, semiring1) in &arc_with_semiring {
                for (arc_rs2, _, sfx2, semiring2) in &arc_with_semiring {
                    if semiring1 == sr1 && semiring2 == sr2 {
                        let base_arc1 = arc_rs1.split("::").last().unwrap();
                        let base_arc2 = arc_rs2.split("::").last().unwrap();
                        let arc_cpp1 = format!("fst::{}", base_arc1);
                        let arc_cpp2 = format!("fst::{}", base_arc2);

                        cpp.push_str(&format!(
                            "    M({}, {}, {}, {}) \\\n",
                            sfx1, arc_cpp1, sfx2, arc_cpp2
                        ));
                    }
                }
            }
        }
        cpp.push('\n');

        std::fs::write(out_dir.join("generated_macros.h"), cpp)
            .expect("Failed to write generated_macros.h");

        let mut rs_macro = String::new();
        rs_macro.push_str("// AUTOGENERATED BY build.rs - DO NOT EDIT\n");
        rs_macro.push_str("#[macro_export]\n");
        rs_macro.push_str("macro_rules! for_each_convertible_arc_pair {\n");
        rs_macro.push_str("    ($user_macro:path) => {\n");
        for (sr1, sr2) in &convertible_semirings {
            for (arc1, _, sfx1, semiring1) in &arc_with_semiring {
                for (arc2, _, sfx2, semiring2) in &arc_with_semiring {
                    if semiring1 == sr1 && semiring2 == sr2 {
                        rs_macro.push_str(&format!(
                            "        $user_macro!({}, {}, {}, {});\n",
                            arc1, arc2, sfx1, sfx2
                        ));
                    }
                }
            }
        }
        rs_macro.push_str("    };\n");
        rs_macro.push_str("}\n");

        std::fs::write(out_dir.join("generated_macros.rs"), rs_macro)
            .expect("Failed to write generated_macros.rs");
    }

    map
}

fn expand_template(
    template: &str,
    arc_macros: &HashMap<String, Vec<Vec<(String, String)>>>,
) -> String {
    let re_expand = Regex::new(r"#\[expand\(([\w_]+)\)\]\s*([^;\{]*(?:;|\{[\s\S]*?\}))").unwrap();

    re_expand
        .replace_all(template, |caps: &regex::Captures| {
            let macro_name = &caps[1];
            let target_code = &caps[2];

            let mut expanded = String::new();
            if let Some(replacements_list) = arc_macros.get(macro_name) {
                for replacements in replacements_list {
                    let mut replaced = target_code.to_string();

                    for (key, val) in replacements {
                        replaced = replaced.replace(key, val);
                    }

                    expanded.push_str(&replaced);
                    expanded.push('\n');
                }
            } else {
                expanded.push_str(target_code);
                expanded.push('\n');
            }
            expanded
        })
        .to_string()
}

fn extract_template_parts(content: &str) -> Option<(String, String, String)> {
    let re = Regex::new(r#"#\[template\("([^"]+)"\)\]\s*(pub\s+mod\s+\w+\s*)\{"#).unwrap();
    let caps = re.captures(content)?;

    let template_name = caps[1].to_string();
    let mod_decl = caps[2].to_string();

    let start_idx = caps.get(0).unwrap().end() - 1; // '{'
    let target_content = &content[start_idx..];

    let mut depth = 0;
    let mut end_idx = 0;
    let mut found_brace = false;

    for (i, c) in target_content.char_indices() {
        if c == '{' {
            depth += 1;
            found_brace = true;
        } else if c == '}' {
            depth -= 1;
            if found_brace && depth == 0 {
                end_idx = i;
                break;
            }
        }
    }

    if found_brace && depth == 0 {
        let inner_body = target_content[1..end_idx].to_string();
        Some((template_name, mod_decl, inner_body))
    } else {
        None
    }
}
