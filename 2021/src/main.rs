use anyhow::{self, Ok};

mod day_1;
mod day_2;
pub mod file_manager;
use day_1::solution_1::solution_1;
use day_2::solution_2::solution_2;

fn main() -> anyhow::Result<()> {
  solution_1()?;
  solution_2()?;
  return Ok(());
}
