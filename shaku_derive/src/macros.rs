//! Implementations of the proc macros

mod common_output;
pub mod component;
pub mod module;
pub mod provider;

#[cfg(feature = "async_provider")]
pub mod async_provider;
