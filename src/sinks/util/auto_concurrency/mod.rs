//! Limit the max number of requests being concurrently processed.

mod controller;
mod future;
mod layer;
mod service;

pub(crate) use self::{layer::AutoConcurrencyLimitLayer, service::AutoConcurrencyLimit};
