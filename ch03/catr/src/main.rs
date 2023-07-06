use catr::Config;
use clap::Parser;

fn main() {
  let config = Config::parse();

  if let Err(e) = catr::run(config) {
    eprint!("{}", e);
    std::process::exit(1);
  }
}
