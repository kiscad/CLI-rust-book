use clap::{Parser, ValueEnum};
use regex::Regex;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, PartialEq, Eq, ValueEnum, Clone, Copy)]
enum EntryType {
  D, // dir
  F, // File
  L, // Link,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Config {
  #[arg(default_values_t = vec![".".to_string()])]
  paths: Vec<String>,
  #[arg(long = "type", short, help = "Entry type")]
  types: Vec<EntryType>,
  #[arg(long = "name", short, help = "name")]
  names: Vec<Regex>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  // dbg!(&config);

  let type_filter = |ent: &DirEntry| {
    config.types.is_empty()
      || config.types.iter().any(|typ| match typ {
        EntryType::D => ent.file_type().is_dir(),
        EntryType::F => ent.file_type().is_file(),
        EntryType::L => ent.path_is_symlink(),
      })
  };

  let name_filter = |ent: &DirEntry| {
    config.names.is_empty()
      || config
        .names
        .iter()
        .any(|re| re.is_match(&ent.file_name().to_string_lossy()))
  };

  for path in config.paths {
    let entries = WalkDir::new(path)
      .into_iter()
      .filter_map(|ent| {
        ent
          .map_err(|e| {
            eprintln!("{e}");
            e
          })
          .ok()
      })
      .filter(type_filter)
      .filter(name_filter)
      .map(|ent| ent.path().display().to_string())
      .collect::<Vec<_>>();
    println!("{}", entries.join("\n"));
  }
  Ok(())
}
