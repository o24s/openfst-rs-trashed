/// Options for controlling caching behavior in delayed FSTs
/// (e.g., ComposeFst, ArcMapFst).
#[derive(Debug, Clone)]
pub struct CacheOptions {
    /// Enables Garbage Collection (GC) of the cache.
    pub gc: bool,
    /// Number of bytes allowed before GC is triggered.
    pub gc_limit: usize,
}

impl Default for CacheOptions {
    fn default() -> Self {
        Self {
            gc: true,    // Default matching OpenFst's FLAGS_fst_default_cache_gc
            gc_limit: 0, // Typically means "use default limit" or no tight limit
        }
    }
}
