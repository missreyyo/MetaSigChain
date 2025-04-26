#![no_std]

mod admin;
mod allowance;
mod balance;
mod contract;
mod metadata;
mod storage_types;

#[cfg(test)]
mod test;

// Re-export the Token struct and TokenClient for external users
pub use crate::contract::Token;
pub use crate::contract::TokenClient;