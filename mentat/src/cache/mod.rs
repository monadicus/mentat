//! Defines an API to implement a custom cache.

mod cache_struct;
pub use cache_struct::*;

mod cache_inner_trait;
pub use cache_inner_trait::*;

mod default_inner_cache;
pub use default_inner_cache::*;
