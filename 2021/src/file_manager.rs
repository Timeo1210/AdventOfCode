use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::{Ok, Result};

pub fn read_file_to_buffer(path: PathBuf) -> Result<BufReader<File>> {
  let file = File::open(path)?;
  let buffer = BufReader::new(file);

  return Ok(buffer);
}
