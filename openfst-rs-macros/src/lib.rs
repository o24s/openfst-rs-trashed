extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn template(attr: TokenStream, _item: TokenStream) -> TokenStream {
    let filename = attr.to_string().replace("\"", "");

    let output = format!(
        "include!(concat!(env!(\"OUT_DIR\"), \"/bridge_{}.rs\"));",
        filename
    );

    output.parse().unwrap()
}
