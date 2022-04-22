//! Shaku is a compile time dependency injection library. It can be used directly or through
//! integration with application frameworks such as [Rocket] (see
//! [`shaku_rocket`]).
//!
//! # Getting started
//! See the [getting started guide]
//!
//! # Crate features
//! By default shaku is thread-safe and exposes macros, but these can be disabled by opting out of
//! the following features:
//!
//! - `thread_safe`: Requires components to be `Send + Sync`
//! - `derive`: Uses the `shaku_derive` crate to provide proc-macro derives of `Component` and
//!   `Provider`, and the `module` macro.
//!
//! [Rocket]: https://rocket.rs
//! [`shaku_rocket`]: https://crates.io/crates/shaku_rocket
//! [getting started guide]: guide/index.html

// This lint is ignored because proc-macros aren't allowed in statement position
// (at least until 1.45). Removing the main function makes rustdoc think the
// module macro is a statement instead of top-level item.
// This can be removed once the MSRV is at least 1.45.
#![allow(clippy::needless_doctest_main)]

// Modules
#[macro_use]
mod trait_alias;
mod component;
mod module;
mod parameters;
mod provider;

pub mod guide;

// Reexport proc macros
#[cfg(all(feature = "derive", feature = "async_provider"))]
pub use shaku_derive::AsyncProvider;
#[cfg(feature = "derive")]
pub use {shaku_derive::module, shaku_derive::Component, shaku_derive::Provider};

// Reexport OnceCell to support lazy components
#[doc(hidden)]
#[cfg(feature = "thread_safe")]
pub use once_cell::sync::OnceCell;
#[doc(hidden)]
#[cfg(not(feature = "thread_safe"))]
pub use once_cell::unsync::OnceCell;

#[doc(hidden)]
#[cfg(not(feature = "anyhow_error"))]
pub type BoxedError = Box<dyn std::error::Error>;
#[doc(hidden)]
#[cfg(all(not(feature = "anyhow_error"), feature = "async_provider"))]
pub type BoxedSendableError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[doc(hidden)]
#[cfg(feature = "anyhow_error")]
pub type BoxedError = anyhow::Error;
#[doc(hidden)]
#[cfg(all(feature = "anyhow_error", feature = "async_provider"))]
pub type BoxedSendableError = anyhow::Error;

// Expose a flat module structure
pub use crate::{component::*, module::*, provider::*};
