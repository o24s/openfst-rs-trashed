#include "cpp/symbol-table.h"
#include <string>
#include <string_view>

namespace fst_rust {

// cxx の rust::Str を OpenFst が期待する absl::string_view / std::string に変換
inline std::string_view to_string_view(rust::Str str) {
    return std::string_view(str.data(), str.size());
}
inline std::string to_std_string(rust::Str str) {
    return std::string(str.data(), str.size());
}

std::unique_ptr<fst::SymbolTable> create_symbol_table(rust::Str name) {
    return std::make_unique<fst::SymbolTable>(to_string_view(name));
}

std::unique_ptr<fst::SymbolTable> copy_symbol_table(const fst::SymbolTable& table) {
    return std::unique_ptr<fst::SymbolTable>(table.Copy());
}

std::unique_ptr<fst::SymbolTable> read_symbol_table(rust::Str source) {
    return std::unique_ptr<fst::SymbolTable>(fst::SymbolTable::Read(to_std_string(source)));
}

std::unique_ptr<fst::SymbolTable> read_symbol_table_text(rust::Str source) {
    return std::unique_ptr<fst::SymbolTable>(fst::SymbolTable::ReadText(to_std_string(source)));
}

bool write_symbol_table(const fst::SymbolTable& table, rust::Str source) {
    return table.Write(to_std_string(source));
}

bool write_symbol_table_text(const fst::SymbolTable& table, rust::Str source) {
    return table.WriteTextWithStatus(to_std_string(source)).ok();
}

int64_t add_symbol(fst::SymbolTable& table, rust::Str symbol, int64_t key) {
    return table.AddSymbol(to_string_view(symbol), key);
}

int64_t add_symbol_auto(fst::SymbolTable& table, rust::Str symbol) {
    return table.AddSymbol(to_string_view(symbol));
}

void remove_symbol(fst::SymbolTable& table, int64_t key) {
    table.RemoveSymbol(key);
}

int64_t find_key(const fst::SymbolTable& table, rust::Str symbol) {
    return table.Find(to_string_view(symbol));
}

rust::String find_symbol(const fst::SymbolTable& table, int64_t key) {
    return rust::String(table.Find(key));
}

bool member_key(const fst::SymbolTable& table, int64_t key) {
    return table.Member(key);
}

bool member_symbol(const fst::SymbolTable& table, rust::Str symbol) {
    return table.Member(to_string_view(symbol));
}

rust::String get_name(const fst::SymbolTable& table) {
    return rust::String(table.Name());
}

void set_name(fst::SymbolTable& table, rust::Str name) {
    table.SetName(to_string_view(name));
}

size_t num_symbols(const fst::SymbolTable& table) {
    return table.NumSymbols();
}

int64_t available_key(const fst::SymbolTable& table) {
    return table.AvailableKey();
}

int64_t get_nth_key(const fst::SymbolTable& table, ssize_t pos) {
    return table.GetNthKey(pos);
}

}  // namespace fst_rust
