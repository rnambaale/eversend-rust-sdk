//! Rust SDK for interacting with the [Eversend](https://eversend.co/) API.

// #![warn(missing_docs)] // TODO: Uncomment this after fully documenting the crate.

mod core;
mod eversend;

pub mod accounts;
pub mod auth;
pub mod beneficiaries;
pub mod collections;
pub mod crypto;
pub mod exchange;
pub mod payouts;
pub mod transactions;
pub mod wallets;

pub use crate::core::*;
pub use crate::eversend::*;
