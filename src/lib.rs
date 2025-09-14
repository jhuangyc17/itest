//! This crate provides several utilities to make examining the result of Rust
//! integration test more convenient.
//!
//! # Example Usage
//! ```rust
//! fn should_pass() {
//!   let dir: PathBuf = new_test_dir!();
//!
//!   // Uses `dir` as output directory for the rust binary under test.
//! }
//! ```

mod cmd;
mod test_dir;

pub use crate::cmd::{CargoCmd, Cmd};
pub use crate::test_dir::TestDir;
pub use color_eyre::eyre::Result;
