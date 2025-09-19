//! This an example rust binary to help with illustrating the usage of
//! [itest::ITest].
//!
//! Usage:
//! ```sh
//! > example_bin arg1 arg2
//! Hello arg1, arg2!
//! Hello World!
//! ```

fn main() {
  let args: Vec<String> = std::env::args().collect();

  println!("Hello {}, {}!", args[1], args[2]);
  eprintln!("Hello World!");
}
