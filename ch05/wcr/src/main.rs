use clap::Parser;
use wcr::Config;

fn main() {
  let config = Config::parse().post_parse();
  if let Err(e) = wcr::run(config) {
    eprintln!("{e}");
    std::process::exit(1);
  }
}
