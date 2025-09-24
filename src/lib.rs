//! This crate provides utility for Rust integration tests to:
//!
//! 1. Create semi-persistent output directory for each test case to operate on
//! 2. Execute command and capture outputs
//!
//! See [ITest] for example usage.

mod itest;

pub use crate::itest::{CmdOutput, ITest};
pub use color_eyre::eyre::Result;
pub use stdext;
