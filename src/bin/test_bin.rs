fn main() {
  let args: Vec<String> = std::env::args().collect();

  println!("Hello {}, {}!", args[1], args[2]);
  eprintln!("Hello World!");
}
