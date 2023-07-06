use clap::Parser;
use std::io;
use std::{
  error::Error,
  fs::File,
  io::{BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
  #[arg(default_values_t = vec!["-".to_string()])]
  files: Vec<String>,
  #[arg(group = "line-number", short = 'n', long = "number")]
  num_lines: bool,
  #[arg(group = "line-number", short = 'b', long = "number-nonblank")]
  num_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
  for fname in &config.files {
    match open(fname) {
      Err(err) => eprintln!("Failed to open {}: {}", fname, err),
      Ok(reader) => {
        let mut n = 1;
        for line in reader.lines() {
          let line = line?;
          if config.num_lines || (config.num_nonblank_lines && line.trim() != "") {
            println!("{:>6}\t{}", n, line);
            n += 1;
          } else {
            println!("{}", line);
          }
        }
      }
    }
  }
  Ok(())
}

fn open(fname: &str) -> MyResult<Box<dyn BufRead>> {
  match fname {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(fname)?))),
  }
}
