use clap::Parser;
use std::{
  error::Error,
  fs::File,
  io::{self, BufRead, BufReader},
};

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

#[derive(Debug, PartialEq, Default)]
pub struct FileInfo {
  num_lines: usize,
  num_words: usize,
  num_bytes: usize,
  num_chars: usize,
}

impl std::ops::AddAssign for FileInfo {
  fn add_assign(&mut self, rhs: Self) {
    self.num_lines += rhs.num_lines;
    self.num_words += rhs.num_words;
    self.num_bytes += rhs.num_bytes;
    self.num_chars += rhs.num_chars;
  }
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  let mut total = FileInfo::default();
  for fpath in &config.files {
    match open(fpath) {
      Err(e) => eprintln!("{fpath}: {e}"),
      Ok(file) => {
        let info = count(file)?;
        println!(
          "{}{}{}{}{}",
          format_field(info.num_lines, config.lines),
          format_field(info.num_words, config.words),
          format_field(info.num_bytes, config.bytes),
          format_field(info.num_chars, config.chars),
          if fpath == "-" {
            "".to_string()
          } else {
            format!(" {fpath}")
          }
        );
        total += info;
      }
    }
  }
  if config.files.len() > 1 {
    println!(
      "{}{}{}{} total",
      format_field(total.num_lines, config.lines),
      format_field(total.num_words, config.words),
      format_field(total.num_bytes, config.bytes),
      format_field(total.num_chars, config.chars)
    );
  }
  Ok(())
}

fn format_field(val: usize, show: bool) -> String {
  if show {
    format!("{val:>8}")
  } else {
    "".to_string()
  }
}

fn open(fpath: &str) -> MyResult<Box<dyn BufRead>> {
  match fpath {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(fpath)?))),
  }
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
  let mut num_lines = 0;
  let mut num_words = 0;
  let mut num_bytes = 0;
  let mut num_chars = 0;

  let mut line = String::new();
  loop {
    let sz = file.read_line(&mut line)?;
    if sz == 0 {
      break;
    }
    num_bytes += sz;
    num_lines += 1;
    num_words += line.split_whitespace().count();
    num_chars += line.chars().count();
    line.clear();
  }

  Ok(FileInfo {
    num_lines,
    num_words,
    num_bytes,
    num_chars,
  })
}

#[cfg(test)]
mod tests {
  use super::{count, FileInfo};
  use std::io::Cursor;

  #[test]
  fn test_count() {
    let text = "I don't want the world. I just want your half.\r\n";
    let info = count(Cursor::new(text));
    assert!(info.is_ok());
    let expected = FileInfo {
      num_lines: 1,
      num_words: 10,
      num_chars: 48,
      num_bytes: 48,
    };
    assert_eq!(info.unwrap(), expected);
  }
}
