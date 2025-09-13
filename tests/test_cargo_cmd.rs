extern crate itest;

use itest::{CargoCmd, Result, TestDir, new_test_dir};

#[test]
fn should_run_cargo_bin() -> Result<()> {
  let dir: TestDir = new_test_dir!()?;

  let cmd = CargoCmd::exec(&dir.path, "id1", "test_bin", &["arg1", "arg2"])?;

  assert_eq!(cmd.stdout, "Hello arg1, arg2!\n");
  assert_eq!(cmd.stderr, "Hello World!\n");

  assert_eq!(
    std::fs::read_to_string(&cmd.stdout_file)?,
    "Hello arg1, arg2!\n"
  );
  assert_eq!(std::fs::read_to_string(&cmd.stderr_file)?, "Hello World!\n");

  Ok(())
}
