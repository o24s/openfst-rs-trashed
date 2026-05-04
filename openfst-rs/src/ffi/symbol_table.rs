#[cxx::bridge(namespace = "fst_rust")]
mod bridge {
    unsafe extern "C++" {
        include!("openfst-rs/cpp/symbol-table.h");

        type SymbolTable;

        fn create_symbol_table(name: &str) -> UniquePtr<SymbolTable>;
        fn copy_symbol_table(table: &SymbolTable) -> UniquePtr<SymbolTable>;

        fn read_symbol_table(source: &str) -> UniquePtr<SymbolTable>;
        fn read_symbol_table_text(source: &str) -> UniquePtr<SymbolTable>;

        fn write_symbol_table(table: &SymbolTable, source: &str) -> bool;
        fn write_symbol_table_text(table: &SymbolTable, source: &str) -> bool;

        fn add_symbol(table: Pin<&mut SymbolTable>, symbol: &str, key: i64) -> i64;
        fn add_symbol_auto(table: Pin<&mut SymbolTable>, symbol: &str) -> i64;
        fn remove_symbol(table: Pin<&mut SymbolTable>, key: i64);

        fn find_key(table: &SymbolTable, symbol: &str) -> i64;
        fn find_symbol(table: &SymbolTable, key: i64) -> String;

        fn member_key(table: &SymbolTable, key: i64) -> bool;
        fn member_symbol(table: &SymbolTable, symbol: &str) -> bool;

        fn get_name(table: &SymbolTable) -> String;
        fn set_name(table: Pin<&mut SymbolTable>, name: &str);

        fn num_symbols(table: &SymbolTable) -> usize;
        fn available_key(table: &SymbolTable) -> i64;
        fn get_nth_key(table: &SymbolTable, pos: isize) -> i64;
    }
}

pub use bridge::*;
