use clap::Parser;
use findr::Config;

fn main() {
  let config = Config::parse();

  if let Err(e) = findr::run(config) {
    eprint!("{e}");
    std::process::exit(1);
  }
}
