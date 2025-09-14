use color_eyre::eyre::{Result, WrapErr};
use std::path::PathBuf;

/// Creates an empty directory under `target/test_output` for an integration
/// test case to use.
///
/// For example, given an integration test file `tests/test1.rs` with teh
/// following test case
///
/// ```rust
/// #[test]
/// fn should_pass() {
///   let dir = new_test_dir!();
/// }
/// ```
///
/// When `new_test_dir!` executes within `should_pass`, it will create an empty
/// directory `target/test_output/test1::should_pass` for the test case to use.
/// When the test finishes, this directory remains available for manual
/// inspection later on.
#[macro_export]
macro_rules! new_test_dir {
  () => {
    TestDir::new(stdext::function_name!())
  };
}

#[derive(Debug)]
pub struct TestDir {
  pub path: PathBuf,
}

impl TestDir {
  pub fn new(test_name: &str) -> Result<TestDir> {
    let dir: PathBuf = std::path::absolute(
      project_root::get_project_root()?
        .join("target")
        .join("test_output")
        .join(test_name),
    )?;

    if std::fs::exists(&dir)? {
      std::fs::remove_dir_all(&dir).wrap_err_with(|| {
        format!("Failed to remove existing test_output dir: {dir:?}")
      })?;
      println!("Removed existing test_output dir: {dir:?}");
    }
    std::fs::create_dir_all(&dir)
      .wrap_err_with(|| format!("Failed to create test output dir: {dir:?}"))?;
    println!(
      "Creating test_output dir: file://{}",
      dir.to_str().expect("valid path")
    );

    Ok(TestDir { path: dir })
  }
}
