use std::{io::BufRead, path::PathBuf};

use crate::file_manager::read_file_to_buffer;
use anyhow::{self, Ok};

fn retrieve_data() -> anyhow::Result<Vec<i32>> {
  let path = PathBuf::from("./src/day_1/input.txt");
  let buffer = read_file_to_buffer(path)?;
  let data: Vec<i32> = buffer
    .lines()
    .map(|x| x.unwrap().parse::<i32>().unwrap())
    .collect();

  return Ok(data);
}

fn solution_1_part_1() -> anyhow::Result<i32> {
  let data = retrieve_data()?;

  let mut last_value: &i32 = &data[0];
  let mut larger_than_previous = 0;
  for value in data.iter() {
    if value > last_value {
      larger_than_previous = larger_than_previous + 1;
    }
    last_value = value;
  }

  return Ok(larger_than_previous);
}

fn solution_1_part_2() -> anyhow::Result<i32> {
  let data = retrieve_data()?;

  let mut larger_than_previous = 0;
  for i in (0..data.len()).step_by(1) {
    if i + 3 >= data.len() {
      break;
    }
    let mesure_a = data[i] + data[i + 1] + data[i + 2];
    let mesure_b = data[i + 1] + data[i + 2] + data[i + 3];
    if mesure_b > mesure_a {
      larger_than_previous = larger_than_previous + 1
    }
  }

  return Ok(larger_than_previous);
}

pub fn solution_1() -> anyhow::Result<()> {
  let answer1 = solution_1_part_1()?;
  let answer2 = solution_1_part_2()?;
  println!(
    "Solution 1, Answers = {{ (Part 1): {} ; (Part 2): {} }}",
    answer1, answer2
  );
  return Ok(());
}
