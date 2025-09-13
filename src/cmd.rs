use assert_cmd::cargo::CommandCargoExt;
use color_eyre::eyre::Result;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output};

pub struct Cmd {
  pub cmd_str: String,
  pub status: ExitStatus,
  pub stdout: String,
  pub stderr: String,
}

impl Cmd {
  pub fn exec(bin: &str, args: &[&str]) -> Result<Self> {
    let mut cmd = Command::new(bin);
    let output: Output = cmd.args(args).output()?;

    Ok(Self {
      cmd_str: format!("{} {}", bin, args.join(" ")),
      status: output.status,
      stdout: String::from_utf8(output.stdout)?,
      stderr: String::from_utf8(output.stderr)?,
    })
  }
}

pub struct CargoCmd {
  pub id: &'static str,
  pub cmd_str: String,
  pub status: ExitStatus,
  pub stdout: String,
  pub stderr: String,
  pub stdout_file: PathBuf,
  pub stderr_file: PathBuf,
}

impl CargoCmd {
  pub fn exec(
    output_dir: &Path,
    id: &'static str,
    bin: &str,
    args: &[&str],
  ) -> Result<Self> {
    let wd: String = Cmd::exec("pwd", &[])?.stdout;

    let cmd_str = format!("cargo run --bin {} -- {}", bin, args.join(" "));
    println!(">>> CargoCmd:\n{cmd_str}\nat wd: {wd}");

    let output: Output = Command::cargo_bin(bin)?.args(args).output()?;
    let stdout: String = String::from_utf8(output.stdout)?;
    println!(">>> Stdout:\n{stdout}\n");
    let stderr: String = String::from_utf8(output.stderr)?;
    println!(">>> Stderr:\n{stderr}\n");

    let stdout_file: PathBuf =
      output_dir.join(format!("CargoCmd_{id}.stdout.txt"));
    std::fs::write(&stdout_file, &stdout)?;

    let stderr_file: PathBuf =
      output_dir.join(format!("CargoCmd_{id}.stderr.txt"));
    std::fs::write(&stderr_file, &stderr)?;

    Ok(Self {
      id,
      cmd_str,
      status: output.status,
      stdout,
      stderr,
      stdout_file,
      stderr_file,
    })
  }
}
