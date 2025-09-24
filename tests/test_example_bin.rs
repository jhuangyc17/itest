use assertables::{assert_ends_with, assert_starts_with};
use itest::{CmdOutput, ITest, Result, new_itest};
use std::path::PathBuf;

#[test]
fn should_run_example_bin() -> Result<()> {
  //
  // This is the expected test directory
  //
  let test_dir: PathBuf =
    std::path::absolute(project_root::get_project_root()?)?
      .join("target/itest_output/test_example_bin::should_run_example_bin");

  //
  // Clean up results from previosu run of this test, if any
  //
  if std::fs::exists(&test_dir)? {
    std::fs::remove_dir_all(&test_dir)?;
  }

  //
  // Initialize ITest
  // Normally, this should be the first line of each test case
  //
  let itest: ITest = new_itest!()?;

  //
  // Path of output dir should be as expected
  assert_eq!(itest.test_output_dir(), test_dir);
  assert_eq!(itest.test_output_dir_str(), test_dir.to_str().unwrap());

  //
  // Empty test directory is created up on creation of ITest
  //
  assert!(std::fs::exists(&test_dir)?);
  assert_eq!(std::fs::read_dir(&test_dir)?.count(), 0);

  //
  // Run a rust binary
  //
  let cmd: CmdOutput =
    itest.exec_cargo("id1", "example_bin", &["arg1", "arg2"])?;

  //
  // stdout and stderr are captured
  //
  assert_eq!(cmd.stdout, "Hello arg1, arg2!\n");
  assert_eq!(cmd.stderr, "Hello World!\n");

  //
  // and also written to files in the test directory
  //
  assert_starts_with!(cmd.stdout_file, test_dir);
  assert_ends_with!(cmd.stdout_file, "Cmd_id1.stdout.txt");
  assert_eq!(
    std::fs::read_to_string(&cmd.stdout_file)?,
    "Hello arg1, arg2!\n"
  );
  assert_starts_with!(cmd.stderr_file, test_dir);
  assert_ends_with!(cmd.stderr_file, "Cmd_id1.stderr.txt");
  assert_eq!(std::fs::read_to_string(&cmd.stderr_file)?, "Hello World!\n");

  Ok(())
}
