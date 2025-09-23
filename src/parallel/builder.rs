//! Parallel operation builder for safe, declarative parallel processing
//!
//! This module provides a builder pattern for creating parallel operations,
//! similar to how bevy-plugin-builder works for plugins.

use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::panic;
use std::sync::Arc;

use super::chunk_size::ChunkStrategy;
use super::safety::{ParallelSafetyError, SafetyValidator};

/// Main entry point for creating parallel operations
pub struct ParallelOperation;

impl ParallelOperation {
    /// Create a new parallel operation with a descriptive name
    pub fn new(operation_name: &str) -> ParallelOperationBuilder<(), ()> {
        ParallelOperationBuilder {
            operation_name: operation_name.to_string(),
            chunk_strategy: ChunkStrategy::Auto,
            safety_validator: SafetyValidator::default(),
            panic_handler: None,
            _phantom: PhantomData,
        }
    }
}

/// Builder for configuring parallel operations
pub struct ParallelOperationBuilder<T, R> {
    operation_name: String,
    chunk_strategy: ChunkStrategy,
    safety_validator: SafetyValidator,
    panic_handler: Option<Box<dyn Fn(&panic::PanicInfo) + Send + Sync>>,
    _phantom: PhantomData<(T, R)>,
}

impl<T, R> ParallelOperationBuilder<T, R>
where
    T: Send + Sync,
    R: Send,
{
    /// Set the data to process in parallel
    pub fn data<D>(self, data: D) -> DataBuilder<D, T, R>
    where
        D: Send + Sync,
    {
        DataBuilder {
            data,
            builder: self,
        }
    }

    /// Enable O(n²) complexity detection
    pub fn validate_no_linear_search(mut self) -> Self {
        self.safety_validator.enable_quadratic_detection();
        self
    }

    /// Set chunk size strategy
    pub fn chunk_size(mut self, strategy: ChunkStrategy) -> Self {
        self.chunk_strategy = strategy;
        self
    }

    /// Set a custom panic handler
    pub fn panic_handler<F>(mut self, handler: F) -> Self
    where
        F: Fn(&panic::PanicInfo) + Send + Sync + 'static,
    {
        self.panic_handler = Some(Box::new(handler));
        self
    }
}

/// Builder stage after data has been set
pub struct DataBuilder<D, T, R> {
    data: D,
    builder: ParallelOperationBuilder<T, R>,
}

impl<D> DataBuilder<D, (), ()>
where
    D: IntoParallelIterator + Send + Sync,
    D::Item: Send,
{
    /// Map operation over the data
    pub fn map<F, R>(self, operation: F) -> ExecutableOperation<D, R>
    where
        F: Fn(D::Item) -> R + Send + Sync + 'static,
        R: Send,
    {
        ExecutableOperation {
            data: self.data,
            operation: Box::new(move |data| {
                data.into_par_iter()
                    .map(operation)
                    .collect()
            }),
            builder: self.builder,
        }
    }

    /// Create a lookup map for O(1) access during parallel operations
    pub fn with_lookup_map<K, V, I, F>(
        self,
        items: I,
        key_fn: F,
    ) -> LookupMapBuilder<D, K, V>
    where
        I: IntoIterator<Item = V>,
        F: Fn(&V) -> K,
        K: Hash + Eq + Send + Sync,
        V: Send + Sync,
    {
        let lookup_map: HashMap<K, V> = items
            .into_iter()
            .map(|item| (key_fn(&item), item))
            .collect();

        LookupMapBuilder {
            data: self.data,
            lookup_map: Arc::new(lookup_map),
            builder: self.builder,
        }
    }
}

/// Builder stage for operations with a lookup map
pub struct LookupMapBuilder<D, K, V> {
    data: D,
    lookup_map: Arc<HashMap<K, V>>,
    builder: ParallelOperationBuilder<(), ()>,
}

impl<D, K, V> LookupMapBuilder<D, K, V>
where
    D: IntoParallelIterator + Send + Sync,
    D::Item: Send,
    K: Hash + Eq + Send + Sync,
    V: Send + Sync,
{
    /// Execute a parallel operation with access to the lookup map
    pub fn parallel_map<F, R>(self, operation: F) -> ExecutableOperation<D, R>
    where
        F: Fn(D::Item, &HashMap<K, V>) -> R + Send + Sync + 'static,
        R: Send,
    {
        let map = self.lookup_map;
        ExecutableOperation {
            data: self.data,
            operation: Box::new(move |data| {
                let map_ref = Arc::clone(&map);
                data.into_par_iter()
                    .map(move |item| {
                        operation(item, &map_ref)
                    })
                    .collect()
            }),
            builder: self.builder,
        }
    }
}

/// Final stage - ready to execute the parallel operation
pub struct ExecutableOperation<D, R> {
    data: D,
    operation: Box<dyn Fn(D) -> Vec<R> + Send + Sync>,
    builder: ParallelOperationBuilder<(), ()>,
}

impl<D, R> ExecutableOperation<D, R>
where
    D: Send + Sync,
    R: Send,
{
    /// Execute the parallel operation
    pub fn execute(self) -> Result<Vec<R>, ParallelSafetyError> {
        // Run safety validations
        self.builder.safety_validator.validate()?;

        // Log the operation
        log::debug!(
            "Executing parallel operation: {} with {} threads",
            self.builder.operation_name,
            rayon::current_num_threads()
        );

        // Execute with panic handling if configured
        let result = if let Some(panic_handler) = self.builder.panic_handler {
            panic::catch_unwind(panic::AssertUnwindSafe(|| {
                (self.operation)(self.data)
            }))
            .map_err(|panic_info| {
                // Call custom panic handler
                // Note: In real implementation, we'd need proper panic info conversion
                log::error!("Parallel operation panicked: {:?}", panic_info);
                ParallelSafetyError::PanicInParallelOperation
            })?
        } else {
            (self.operation)(self.data)
        };

        Ok(result)
    }
}