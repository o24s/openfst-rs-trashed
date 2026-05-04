use cxx::UniquePtr;
use std::path::Path;

use crate::ffi::symbol_table::{self as cxx_ffi, SymbolTable as CxxSymbolTable};

pub const NO_SYMBOL: i64 = -1;

/// Rust idiomatic wrapper for OpenFst's `fst::SymbolTable`.
pub struct SymbolTable {
    pub(crate) inner: UniquePtr<CxxSymbolTable>,
}

impl SymbolTable {
    /// Creates a new, empty SymbolTable with an optional name.
    pub fn new(name: &str) -> Self {
        Self {
            inner: cxx_ffi::create_symbol_table(name),
        }
    }

    /// Creates a reference-counted (or deep) copy of this symbol table.
    pub fn clone_table(&self) -> Self {
        Self {
            inner: cxx_ffi::copy_symbol_table(&self.inner),
        }
    }

    /// Reads a binary dump of the symbol table from a file.
    pub fn read<P: AsRef<Path>>(path: P) -> Option<Self> {
        let ptr = cxx_ffi::read_symbol_table(path.as_ref().to_str()?);
        if ptr.is_null() {
            None
        } else {
            Some(Self { inner: ptr })
        }
    }

    /// Reads a text representation of the symbol table from a file.
    pub fn read_text<P: AsRef<Path>>(path: P) -> Option<Self> {
        let ptr = cxx_ffi::read_symbol_table_text(path.as_ref().to_str()?);
        if ptr.is_null() {
            None
        } else {
            Some(Self { inner: ptr })
        }
    }

    /// Writes a binary dump of the symbol table to a file.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(p) = path.as_ref().to_str() {
            cxx_ffi::write_symbol_table(&self.inner, p)
        } else {
            false
        }
    }

    /// Writes a text representation of the symbol table to a file.
    pub fn write_text<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(p) = path.as_ref().to_str() {
            cxx_ffi::write_symbol_table_text(&self.inner, p)
        } else {
            false
        }
    }

    /// Adds a symbol with given key to the table.
    pub fn add_symbol(&mut self, symbol: &str, key: i64) -> i64 {
        cxx_ffi::add_symbol(self.inner.pin_mut(), symbol, key)
    }

    /// Adds a symbol to the table. The associated key is automatically assigned.
    pub fn add_symbol_auto(&mut self, symbol: &str) -> i64 {
        cxx_ffi::add_symbol_auto(self.inner.pin_mut(), symbol)
    }

    /// Removes the symbol with the specified key.
    pub fn remove_symbol(&mut self, key: i64) {
        cxx_ffi::remove_symbol(self.inner.pin_mut(), key);
    }

    /// Returns the key associated with the symbol.
    pub fn find_key(&self, symbol: &str) -> Option<i64> {
        let key = cxx_ffi::find_key(&self.inner, symbol);
        if key == NO_SYMBOL { None } else { Some(key) }
    }

    /// Returns the string associated with the key.
    pub fn find_symbol(&self, key: i64) -> Option<String> {
        let sym = cxx_ffi::find_symbol(&self.inner, key);
        if sym.is_empty() { None } else { Some(sym) }
    }

    /// Checks if the given key exists in the table.
    pub fn member_key(&self, key: i64) -> bool {
        cxx_ffi::member_key(&self.inner, key)
    }

    /// Checks if the given symbol exists in the table.
    pub fn member_symbol(&self, symbol: &str) -> bool {
        cxx_ffi::member_symbol(&self.inner, symbol)
    }

    /// Returns the name of the symbol table.
    pub fn name(&self) -> String {
        cxx_ffi::get_name(&self.inner)
    }

    /// Sets the name of the symbol table.
    pub fn set_name(&mut self, name: &str) {
        cxx_ffi::set_name(self.inner.pin_mut(), name);
    }

    /// Returns the current number of symbols in the table.
    pub fn num_symbols(&self) -> usize {
        cxx_ffi::num_symbols(&self.inner)
    }

    /// Returns the current available key (i.e., highest key + 1).
    pub fn available_key(&self) -> i64 {
        cxx_ffi::available_key(&self.inner)
    }

    /// Provides an iterator over (key, symbol) pairs.
    pub fn iter(&self) -> SymbolTableIterator<'_> {
        SymbolTableIterator {
            table: self,
            pos: 0,
            num_symbols: self.num_symbols() as isize,
        }
    }

    pub(crate) unsafe fn from_raw_ptr(ptr: UniquePtr<CxxSymbolTable>) -> Self {
        Self { inner: ptr }
    }

    pub(crate) fn as_raw_ptr(&self) -> *const CxxSymbolTable {
        if self.inner.is_null() {
            std::ptr::null()
        } else {
            &*self.inner as *const CxxSymbolTable
        }
    }
}

/// Iterator for SymbolTable.
pub struct SymbolTableIterator<'a> {
    table: &'a SymbolTable,
    pos: isize,
    num_symbols: isize,
}

impl<'a> Iterator for SymbolTableIterator<'a> {
    type Item = (i64, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.num_symbols {
            return None;
        }
        let key = cxx_ffi::get_nth_key(&self.table.inner, self.pos);
        self.pos += 1;

        let sym = self.table.find_symbol(key).unwrap_or_default();
        Some((key, sym))
    }
}
