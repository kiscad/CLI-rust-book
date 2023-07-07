use clap::Parser;
use headr::Config;

fn main() {
  let config = Config::parse();
  if let Err(e) = headr::run(config) {
    eprintln!("{e}");
    std::process::exit(1);
  }
}
