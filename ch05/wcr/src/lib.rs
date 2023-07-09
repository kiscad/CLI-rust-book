use clap::Parser;
use std::error::Error;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
  #[arg(default_values_t = vec!["-".to_string()], help = "Input file(s)")]
  files: Vec<String>,
  #[arg(long, short, help = "Show line count")]
  lines: bool,
  #[arg(long, short, help = "Show word count")]
  words: bool,
  #[arg(
    long,
    short = 'm',
    group = "byte-or-char",
    help = "Show character count"
  )]
  chars: bool,
  #[arg(long, short = 'c', group = "byte-or-char", help = "Show byte count")]
  bytes: bool,
}

impl Config {
  pub fn post_parse(mut self) -> Self {
    let any_flag = [self.bytes, self.chars, self.lines, self.words]
      .into_iter()
      .any(|x| x);
    if !any_flag {
      self.bytes = true;
      self.lines = true;
      self.words = true;
    }
    self
  }
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  dbg!(config);
  Ok(())
}
