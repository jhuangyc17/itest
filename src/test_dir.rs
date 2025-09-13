use color_eyre::eyre::{Result, WrapErr};
use std::path::PathBuf;

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
