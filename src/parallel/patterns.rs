//! Common parallel processing patterns for Living Worlds
//!
//! This module provides reusable patterns that prevent common bugs
//! and ensure consistent parallel processing across the codebase.

use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

/// Common parallel processing patterns
pub enum ParallelPattern {
    /// Simple map over data
    Map,

    /// Map with reduction
    MapReduce,

    /// Filter then map
    FilterMap,

    /// Process with HashMap lookup for O(1) access
    MapWithLookup,

    /// Parallel sorting
    Sort,

    /// Parallel aggregation
    Aggregate,
}

/// Trait for operations that need a lookup map for O(1) access
pub trait WithLookupMap<K, V>
where
    K: Hash + Eq,
{
    /// Build a HashMap for O(1) lookups during parallel processing
    fn build_lookup_map<I, F>(items: I, key_fn: F) -> Arc<HashMap<K, V>>
    where
        I: IntoIterator<Item = V>,
        F: Fn(&V) -> K;
}

/// Default implementation for building lookup maps
pub struct LookupMapBuilder;

impl<K, V> WithLookupMap<K, V> for LookupMapBuilder
where
    K: Hash + Eq,
{
    fn build_lookup_map<I, F>(items: I, key_fn: F) -> Arc<HashMap<K, V>>
    where
        I: IntoIterator<Item = V>,
        F: Fn(&V) -> K,
    {
        let map: HashMap<K, V> = items
            .into_iter()
            .map(|item| (key_fn(&item), item))
            .collect();
        Arc::new(map)
    }
}

/// Process provinces in parallel with safe patterns
pub fn parallel_process_provinces<T, F>(
    provinces: &[T],
    operation: F,
) -> Vec<T>
where
    T: Clone + Send + Sync,
    F: Fn(&T) -> T + Send + Sync,
{
    provinces
        .par_iter()
        .map(|province| operation(province))
        .collect()
}

/// Parallel aggregation with proper reduction
pub fn parallel_aggregate<T, A, F, R>(
    data: &[T],
    init: A,
    accumulator: F,
    reducer: R,
) -> A
where
    T: Send + Sync,
    A: Send + Sync + Clone,
    F: Fn(A, &T) -> A + Send + Sync,
    R: Fn(A, A) -> A + Send + Sync,
{
    data.par_iter()
        .fold(|| init.clone(), |acc, item| accumulator(acc, item))
        .reduce(|| init.clone(), reducer)
}

/// Parallel filtering with collection
pub fn parallel_filter_collect<T, F>(
    data: &[T],
    predicate: F,
) -> Vec<T>
where
    T: Clone + Send + Sync,
    F: Fn(&T) -> bool + Send + Sync,
{
    data.par_iter()
        .filter(|item| predicate(item))
        .cloned()
        .collect()
}

/// Process with index-based HashMap for guaranteed O(1) lookups
pub fn parallel_with_index_map<T, R, F>(
    data: &[T],
    operation: F,
) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    F: Fn(usize, &T, &HashMap<usize, &T>) -> R + Send + Sync,
{
    // Build index map for O(1) lookups
    let index_map: HashMap<usize, &T> = data
        .iter()
        .enumerate()
        .map(|(i, item)| (i, item))
        .collect();
    let index_map = Arc::new(index_map);

    data.par_iter()
        .enumerate()
        .map(|(i, item)| {
            let map_ref = Arc::clone(&index_map);
            operation(i, item, &map_ref)
        })
        .collect()
}

/// Macro for safe parallel iteration with automatic HashMap building
#[macro_export]
macro_rules! parallel_with_lookup {
    ($data:expr_2021, $lookup_data:expr_2021, $key_fn:expr_2021, |$item:ident, $map:ident| $body:expr_2021) => {{
        use $crate::parallel::patterns::WithLookupMap;
        use $crate::parallel::patterns::LookupMapBuilder;

        let lookup_map = LookupMapBuilder::build_lookup_map($lookup_data, $key_fn);
        $data
            .par_iter()
            .map(|$item| {
                let $map = &*lookup_map;
                $body
            })
            .collect()
    }};
}