//! High-level parallel operations for common Living Worlds patterns
//!
//! This module provides specialized parallel operations tailored to
//! Living Worlds' specific needs, making migration from raw Rayon trivial.

use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use super::chunk_size::ChunkStrategy;
use super::safety::ParallelSafetyError;

/// Parallel map operation for simple transformations
pub fn parallel_map<T, R, F>(
    data: &[T],
    operation: F,
    operation_name: &str,
) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    F: Fn(&T) -> R + Send + Sync,
{
    log::trace!("Parallel map: {} on {} items", operation_name, data.len());

    data.par_iter()
        .map(operation)
        .collect()
}

/// Parallel in-place mutation
pub fn parallel_mutate<T, F>(
    data: &mut [T],
    operation: F,
    operation_name: &str,
) where
    T: Send + Sync,
    F: Fn(&mut T) + Send + Sync,
{
    log::trace!("Parallel mutate: {} on {} items", operation_name, data.len());

    data.par_iter_mut()
        .for_each(operation);
}

/// Parallel zip operation for processing multiple collections together
pub fn parallel_zip<A, B, R, F>(
    data_a: &[A],
    data_b: &[B],
    operation: F,
    operation_name: &str,
) -> Vec<R>
where
    A: Send + Sync,
    B: Send + Sync,
    R: Send,
    F: Fn(&A, &B) -> R + Send + Sync,
{
    log::trace!(
        "Parallel zip: {} on {} + {} items",
        operation_name,
        data_a.len(),
        data_b.len()
    );

    data_a.par_iter()
        .zip(data_b.par_iter())
        .map(|(a, b)| operation(a, b))
        .collect()
}

/// Parallel mutation with zip for updating based on another collection
pub fn parallel_zip_mutate<A, B, F>(
    data_a: &mut [A],
    data_b: &[B],
    operation: F,
    operation_name: &str,
) where
    A: Send + Sync,
    B: Send + Sync,
    F: Fn(&mut A, &B) + Send + Sync,
{
    log::trace!(
        "Parallel zip mutate: {} on {} items",
        operation_name,
        data_a.len()
    );

    data_a.par_iter_mut()
        .zip(data_b.par_iter())
        .for_each(|(a, b)| operation(a, b));
}

/// Parallel enumerated iteration for operations that need indices
pub fn parallel_enumerate<T, R, F>(
    data: &[T],
    operation: F,
    operation_name: &str,
) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    F: Fn(usize, &T) -> R + Send + Sync,
{
    log::trace!(
        "Parallel enumerate: {} on {} items",
        operation_name,
        data.len()
    );

    data.par_iter()
        .enumerate()
        .map(|(i, item)| operation(i, item))
        .collect()
}

/// Parallel operation with pre-built HashMap for O(1) lookups
pub fn parallel_with_lookup<T, K, V, R, F>(
    data: &[T],
    lookup_source: &[V],
    key_extractor: impl Fn(&V) -> K,
    operation: F,
    operation_name: &str,
) -> Vec<R>
where
    T: Send + Sync,
    K: Hash + Eq + Send + Sync,
    V: Send + Sync + Clone,
    R: Send,
    F: Fn(&T, &HashMap<K, V>) -> R + Send + Sync,
{
    log::trace!(
        "Parallel with lookup: {} on {} items with {} lookup entries",
        operation_name,
        data.len(),
        lookup_source.len()
    );

    // Build HashMap for O(1) lookups - prevents O(n²) complexity
    let lookup_map: HashMap<K, V> = lookup_source
        .iter()
        .map(|v| (key_extractor(v), v.clone()))
        .collect();
    let lookup_map = Arc::new(lookup_map);

    data.par_iter()
        .map(|item| {
            let map_ref = Arc::clone(&lookup_map);
            operation(item, &map_ref)
        })
        .collect()
}

/// Parallel chunks processing for cache-efficient batch operations
pub fn parallel_chunks<T, F>(
    data: &[T],
    chunk_strategy: ChunkStrategy,
    operation: F,
    operation_name: &str,
) -> Vec<Vec<T>>
where
    T: Send + Sync + Clone,
    F: Fn(&[T]) -> Vec<T> + Send + Sync,
{
    let chunk_size = chunk_strategy.calculate_chunk_size(data.len());

    log::trace!(
        "Parallel chunks: {} on {} items with chunk size {}",
        operation_name,
        data.len(),
        chunk_size
    );

    data.par_chunks(chunk_size)
        .map(operation)
        .collect()
}

/// Parallel fold with reduction for aggregations
pub fn parallel_fold<T, A, F, R>(
    data: &[T],
    init: A,
    fold_op: F,
    reduce_op: R,
    operation_name: &str,
) -> A
where
    T: Send + Sync,
    A: Send + Clone,
    F: Fn(A, &T) -> A + Send + Sync,
    R: Fn(A, A) -> A + Send + Sync,
{
    log::trace!(
        "Parallel fold: {} on {} items",
        operation_name,
        data.len()
    );

    data.par_iter()
        .fold(|| init.clone(), |acc, item| fold_op(acc, item))
        .reduce(|| init.clone(), reduce_op)
}

/// Parallel filter and map combined
pub fn parallel_filter_map<T, R, P, M>(
    data: &[T],
    predicate: P,
    mapper: M,
    operation_name: &str,
) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    P: Fn(&T) -> bool + Send + Sync,
    M: Fn(&T) -> R + Send + Sync,
{
    log::trace!(
        "Parallel filter map: {} on {} items",
        operation_name,
        data.len()
    );

    data.par_iter()
        .filter(|item| predicate(item))
        .map(mapper)
        .collect()
}

/// Safe parallel iteration for Bevy Query results (prevents ECS conflicts)
pub fn parallel_query_iter<T, F>(
    items: Vec<T>,
    operation: F,
    operation_name: &str,
) where
    T: Send,
    F: Fn(T) + Send + Sync,
{
    log::trace!(
        "Parallel query iter: {} on {} items",
        operation_name,
        items.len()
    );

    // Convert query results to Vec first to avoid ECS conflicts
    items.into_par_iter()
        .for_each(operation);
}