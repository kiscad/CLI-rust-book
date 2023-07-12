use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
  #[arg(default_values_t = vec![".".to_string()])]
  path: Vec<String>,
  #[arg(long = "type", short, help = "Entry type [possible values: f, d l]")]
  types: Vec<char>,
  #[arg(long = "name", short, help = "name")]
  names: Vec<String>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  dbg!(config);
  Ok(())
}
