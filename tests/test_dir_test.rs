extern crate itest;

use itest::{Result, TestDir, new_test_dir};
use std::path::{Path, PathBuf};

#[test]
fn should_create_test_dir() -> Result<()> {
  let dir: TestDir = new_test_dir!()?;

  let partial_dir: &Path =
    dir.path.strip_prefix(project_root::get_project_root()?)?;
  assert_eq!(
    partial_dir,
    PathBuf::from("target/test_output/test_dir_test::should_create_test_dir")
  );

  let test_file: PathBuf = dir.path.join("test1.txt");
  std::fs::write(&test_file, "test1")?;
  let content: String = std::fs::read_to_string(&test_file)?;
  assert_eq!(content, "test1");

  let dir: TestDir = new_test_dir!()?;

  let partial_dir: &Path =
    dir.path.strip_prefix(project_root::get_project_root()?)?;
  assert_eq!(
    partial_dir,
    PathBuf::from("target/test_output/test_dir_test::should_create_test_dir")
  );

  assert!(!std::fs::exists(&test_file)?);

  Ok(())
}
