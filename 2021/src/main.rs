use anyhow::{self, Ok};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
pub mod file_manager;
use day_1::solution_1::solution_1;
use day_2::solution_2::solution_2;
use day_3::solution_3::solution_3;
use day_4::solution_4::solution_4;
use day_5::solution_5::solution_5;
use day_6::solution_6::solution_6;

fn main() -> anyhow::Result<()> {
  solution_1()?;
  solution_2()?;
  solution_3()?;
  solution_4()?;
  solution_5()?;
  solution_6()?;
  return Ok(());
}
