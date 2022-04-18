#![deny(missing_docs)]

//! A minimal Solana program template

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

/// Current program version
pub const PROGRAM_VERSION: u8 = 1;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("6fKUpiC3zAZZ3Y2KsidwECZAivpW4K3NVNJK8UEAC4DR");
