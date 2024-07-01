//! Rust SDK for interacting with the [Eversend](https://eversend.co/) API.

// #![warn(missing_docs)]

mod core;
mod eversend;

pub mod accounts;
pub mod auth;
pub mod wallets;

pub use crate::core::*;
pub use crate::eversend::*;
