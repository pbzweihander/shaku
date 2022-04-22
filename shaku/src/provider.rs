//! This module contains trait definitions for provided services and interfaces

use crate::module::ModuleInterface;
use crate::BoxedError;
use crate::Module;

/// Like [`Component`]s, providers provide a service by implementing an interface.
///
/// Unlike [`Component`], `Provider` represents a temporary service. Examples include a connection
/// to a remote service or pooled database connection. Because only providers can have other
/// providers as dependencies, services which use these provided services must also be `Provider`s
/// (ex. DB repository, service using a DB repository, etc).
///
/// See also the [provider getting started guide].
///
/// [`Component`]: trait.Component.html
/// [provider getting started guide]: guide/provider/index.html
pub trait Provider<M: Module>: 'static {
    /// The trait/interface which this provider implements
    type Interface: ?Sized;

    /// Provides the service, possibly resolving other components/providers
    /// to do so.
    fn provide(module: &M) -> Result<Box<Self::Interface>, BoxedError>;
}

/// The type signature of [`Provider::provide`]. This is used when overriding a
/// provider via [`ModuleBuilder::with_provider_override`]
///
/// [`Provider::provide`]: trait.Provider.html#tymethod.provide
/// [`ModuleBuilder::with_provider_override`]: struct.ModuleBuilder.html#method.with_provider_override
#[cfg(not(feature = "thread_safe"))]
pub type ProviderFn<M, I> = Box<dyn (Fn(&M) -> Result<Box<I>, BoxedError>)>;
/// The type signature of [`Provider::provide`]. This is used when overriding a
/// provider via [`ModuleBuilder::with_provider_override`]
///
/// [`Provider::provide`]: trait.Provider.html#tymethod.provide
/// [`ModuleBuilder::with_provider_override`]: struct.ModuleBuilder.html#method.with_provider_override
#[cfg(feature = "thread_safe")]
pub type ProviderFn<M, I> = Box<dyn (Fn(&M) -> Result<Box<I>, BoxedError>) + Send + Sync>;

/// Indicates that a module contains a provider which implements the interface.
pub trait HasProvider<I: ?Sized>: ModuleInterface {
    /// Create a service using the provider registered with the interface `I`.
    /// Each call will create a new instance of the service.
    ///
    /// # Examples
    /// ```
    /// # use shaku::{module, HasProvider, Provider};
    /// # use std::sync::Arc;
    /// #
    /// # trait Foo {}
    /// #
    /// # #[derive(Provider)]
    /// # #[shaku(interface = Foo)]
    /// # struct FooImpl;
    /// # impl Foo for FooImpl {}
    /// #
    /// # module! {
    /// #     TestModule {
    /// #         components = [],
    /// #         providers = [FooImpl]
    /// #     }
    /// # }
    /// #
    /// # fn main() {
    /// # let module = TestModule::builder().build();
    /// #
    /// let foo: Box<dyn Foo> = module.provide().unwrap();
    /// # }
    /// ```
    fn provide(&self) -> Result<Box<I>, BoxedError>;
}

#[cfg(feature = "async_provider")]
mod async_provider {
    use std::future::Future;
    use std::pin::Pin;

    use super::*;
    use crate::BoxedSendableError;

    pub type AsyncProvideFuture<'a, I> =
        Pin<Box<dyn Future<Output = Result<Box<I>, BoxedSendableError>> + Send + 'a>>;

    pub trait AsyncProvider<M: Module>: 'static {
        type Interface: ?Sized;

        fn async_provide(module: &M) -> AsyncProvideFuture<'_, Self::Interface>;
    }

    pub trait HasAsyncProvider<I: ?Sized>: ModuleInterface {
        fn async_provide(&self) -> AsyncProvideFuture<'_, I>;
    }

    pub type AsyncProviderFn<M, I> =
        Box<dyn (for<'a> Fn(&'a M) -> AsyncProvideFuture<'a, I>) + Send + Sync>;
}

#[cfg(feature = "async_provider")]
pub use async_provider::*;
