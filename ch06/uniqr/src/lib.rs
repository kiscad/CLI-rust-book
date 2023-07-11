use clap::Parser;
use std::{
  error::Error,
  fs::File,
  io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
  #[arg(default_value_t = String::from("-"))]
  in_file: String,
  #[arg(default_value_t = String::from("-"))]
  out_file: String,
  #[arg(short, long, help = "prefix lines by the number of occurrences")]
  count: bool,
}

pub fn run(config: Config) -> MyResult<()> {
  let mut in_file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
  let mut out_file =
    open_output(&config.out_file).map_err(|e| format!("{}: {}", config.out_file, e))?;
  let mut count = 0_usize;
  let mut last_line = String::new();
  let mut line = String::new();
  let mut print = |count, line: &str| -> MyResult<()> {
    if count > 0 {
      if config.count {
        write!(out_file, "{count:>4} {line}")?;
      } else {
        write!(out_file, "{line}")?;
      }
    }
    Ok(())
  };

  loop {
    let bytes = in_file.read_line(&mut line)?;
    if bytes == 0 {
      print(count, &last_line)?;
      out_file.flush()?;
      break;
    }
    if line.trim_end() == last_line.trim_end() {
      count += 1;
    } else {
      print(count, &last_line)?;
      last_line.clear();
      last_line.push_str(&line);
      count = 1;
    }
    line.clear();
  }
  Ok(())
}

fn open_output(fpath: &str) -> MyResult<Box<dyn Write>> {
  match fpath {
    "-" => Ok(Box::new(io::stdout())),
    _ => Ok(Box::new(File::create(fpath)?)),
  }
}

fn open(fpath: &str) -> MyResult<Box<dyn BufRead>> {
  match fpath {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(fpath)?))),
  }
}
