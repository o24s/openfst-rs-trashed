#pragma once
#include "openfst/lib/symbol-table.h"
#include <memory>
#include <cstdint>
#include "rust/cxx.h"

namespace fst_rust {

using SymbolTable = fst::SymbolTable;

std::unique_ptr<fst::SymbolTable> create_symbol_table(rust::Str name);
std::unique_ptr<fst::SymbolTable> copy_symbol_table(const fst::SymbolTable& table);

std::unique_ptr<fst::SymbolTable> read_symbol_table(rust::Str source);
std::unique_ptr<fst::SymbolTable> read_symbol_table_text(rust::Str source);
bool write_symbol_table(const fst::SymbolTable& table, rust::Str source);
bool write_symbol_table_text(const fst::SymbolTable& table, rust::Str source);

int64_t add_symbol(fst::SymbolTable& table, rust::Str symbol, int64_t key);
int64_t add_symbol_auto(fst::SymbolTable& table, rust::Str symbol);
void remove_symbol(fst::SymbolTable& table, int64_t key);

int64_t find_key(const fst::SymbolTable& table, rust::Str symbol);
rust::String find_symbol(const fst::SymbolTable& table, int64_t key);

bool member_key(const fst::SymbolTable& table, int64_t key);
bool member_symbol(const fst::SymbolTable& table, rust::Str symbol);

rust::String get_name(const fst::SymbolTable& table);
void set_name(fst::SymbolTable& table, rust::Str name);
size_t num_symbols(const fst::SymbolTable& table);
int64_t available_key(const fst::SymbolTable& table);

int64_t get_nth_key(const fst::SymbolTable& table, ssize_t pos);

}  // namespace fst_rust
