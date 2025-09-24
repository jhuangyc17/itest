use assert_cmd::cargo::CommandCargoExt;
use color_eyre::eyre::{Result, WrapErr};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output};

/// Creates an [ITest] instance for the enclosing test case.
#[macro_export]
macro_rules! new_itest {
  () => {
    ITest::new($crate::stdext::function_name!())
  };
}

/// The fixture for a test case.
///
/// Example usage:
/// ```rust
#[doc = include_str!("../tests/test_example_bin.rs")]
/// ```
pub struct ITest {
  test_output_dir: PathBuf,
}

/// Contains the result of running a binary under [ITest].
pub struct CmdOutput {
  pub cmd: String,
  pub status: ExitStatus,
  pub stdout: String,
  pub stderr: String,
  pub stdout_file: PathBuf,
  pub stderr_file: PathBuf,
}

impl ITest {
  /// Creates an [ITest] with a given test case name.
  ///
  /// In most case, the macro [new_itest!] should be used instead as it
  /// automatically captures the enclosing test function name.
  pub fn new(test_name: &str) -> Result<Self> {
    println!("ITest> Initializing for test [{test_name}]");

    let test_output_dir: PathBuf = std::path::absolute(
      project_root::get_project_root()?
        .join("target")
        .join("itest_output")
        .join(test_name),
    )?;

    if std::fs::exists(&test_output_dir)? {
      std::fs::remove_dir_all(&test_output_dir).wrap_err_with(|| {
        format!(
          "Failed to remove existing test_output_dir: {test_output_dir:?}"
        )
      })?;
      println!("ITest> Removed existing test_output_dir: {test_output_dir:?}");
    }

    std::fs::create_dir_all(&test_output_dir).wrap_err_with(|| {
      format!("Failed to create test_output_dir: {test_output_dir:?}")
    })?;
    println!(
      "ITest> Created test_output_dir: file://{}",
      test_output_dir.to_str().expect("valid path")
    );

    Ok(Self { test_output_dir })
  }

  /// Path to the test output directory.
  pub fn test_output_dir(&self) -> &Path {
    &self.test_output_dir
  }

  /// Executes a binary.
  pub fn exec(&self, id: &str, bin: &str, args: &[&str]) -> Result<CmdOutput> {
    let mut cmd = Command::new(bin);
    cmd.args(args);

    run_and_collect_output(id, cmd, &self.test_output_dir)
  }

  /// Executes a cargo binary
  pub fn exec_cargo(
    &self,
    id: &str,
    bin: &str,
    args: &[&str],
  ) -> Result<CmdOutput> {
    let mut cmd = Command::cargo_bin(bin)?;
    cmd.args(args);

    run_and_collect_output(id, cmd, &self.test_output_dir)
  }
}

fn run_and_collect_output(
  id: &str,
  // cmd_str: &str,
  mut cmd: Command,
  output_dir: &Path,
) -> Result<CmdOutput> {
  let cmd_str: String = format!("{cmd:?}");
  println!("ITest> Executing: {cmd_str}");
  let output: Output = cmd.output()?;

  let stdout = String::from_utf8(output.stdout)?;
  println!("ITest> Stdout:\n{stdout}\n");

  let stderr = String::from_utf8(output.stderr)?;
  println!("ITest> Stderr:\n{stderr}\n");

  let stdout_file: PathBuf = output_dir.join(format!("Cmd_{id}.stdout.txt"));
  std::fs::write(&stdout_file, &stdout)?;

  let stderr_file: PathBuf = output_dir.join(format!("Cmd_{id}.stderr.txt"));
  std::fs::write(&stderr_file, &stderr)?;

  Ok(CmdOutput {
    cmd: cmd_str,
    status: output.status,
    stdout,
    stderr,
    stdout_file,
    stderr_file,
  })
}
