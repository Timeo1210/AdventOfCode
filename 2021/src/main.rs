use anyhow::{self, Ok};

mod day_1;
pub mod file_manager;
use day_1::solution_1::solution_1;

fn main() -> anyhow::Result<()> {
  solution_1()?;
  return Ok(());
}
