//! This crate provides integration between the `shaku` and `rocket` crates.
//!
//! See [`Inject`] and [`InjectProvided`] for details.
//!
//! [`Inject`]: struct.Inject.html
//! [`InjectProvided`]: struct.InjectProvided.html

mod inject_component;
mod inject_provided;

pub use inject_component::Inject;
pub use inject_provided::InjectProvided;

use rocket::request::Outcome;
use rocket::{Request, State};
use shaku::ModuleInterface;

fn get_module_from_state<'r, M: ModuleInterface + ?Sized>(
    request: &Request<'r>,
) -> Outcome<State<'r, Box<M>>, String> {
    request
        .guard()
        .map_failure(|f| (f.0, "Failed to retrieve module from state".to_string()))
}
