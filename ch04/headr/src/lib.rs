use clap::Parser;
use std::io::{self, Read};
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
  #[arg(group = "mode", short = 'n', long, default_value_t = 10)]
  lines: usize,
  #[arg(group = "mode", short = 'c', long)]
  bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
  let num_files = config.files.len();
  for (i, fname) in config.files.iter().enumerate() {
    match open(fname) {
      Err(e) => eprintln!("{}: {}", fname, e),
      Ok(mut reader) => {
        if num_files > 1 {
          println!("{}==> {} <==", if i > 0 { "\n" } else { "" }, fname);
        }
        if let Some(bytes) = config.bytes {
          let buf: Vec<u8> = reader.bytes().take(bytes).map(|x| x.unwrap()).collect();
          print!("{}", String::from_utf8_lossy(&buf));
        } else {
          let mut line = String::new();
          for _ in 0..config.lines {
            let bytes = reader.read_line(&mut line)?;
            if bytes == 0 {
              break;
            }
            print!("{}", line);
            line.clear();
          }
        }
      }
    }
  }
  Ok(())
}

fn open(fpath: &str) -> MyResult<Box<dyn BufRead>> {
  match fpath {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(fpath)?))),
  }
}
