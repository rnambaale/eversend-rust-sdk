//! Rust SDK for interacting with the [Eversend](https://eversend.co/) API.

mod core;
mod eversend;

pub mod auth;
pub mod wallets;

pub use crate::core::*;
pub use crate::eversend::*;
