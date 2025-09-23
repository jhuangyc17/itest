//! This crate provides utility for Rust integration tests to:
//!
//! 1. Create semi-persistent output directory for each test case to operate on
//! 2. Execute command and capture outputs
//!
//! See [ITest] for example usage.

mod itest;

pub use crate::itest::{CmdOutput, ITest};
pub use color_eyre::eyre::Result;
pub use stdext::function_name;

// The #[cfg(doc)] attribute ensures this module is only compiled for documentation.
// #[cfg(doc)]
// #[doc(hidden)]
// // #[path = "../tests"]
// mod doc_tests {
//   // Include the content of your test file
//   #[doc = include_str!("../tests/test_example_bin.rs")]
//   mod test_example_bin {}
// }
