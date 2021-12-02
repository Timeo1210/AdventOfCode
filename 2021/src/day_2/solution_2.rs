use std::{io::BufRead, path::PathBuf, str::FromStr};

use crate::file_manager::read_file_to_buffer;
use anyhow::{self, Ok, Result};

#[derive(PartialEq)]
enum Mouvements {
  Forward,
  Up,
  Down,
}

impl FromStr for Mouvements {
  type Err = anyhow::Error;

  fn from_str(input: &str) -> Result<Mouvements, Self::Err> {
    match input {
      "down" => Ok(Mouvements::Down),
      "up" => Ok(Mouvements::Up),
      "forward" => Ok(Mouvements::Forward),
      _ => Err(anyhow::anyhow!("Invalid")),
    }
  }
}

struct Action {
  mouvement: Mouvements,
  amount: u32,
}

fn retrieve_data() -> anyhow::Result<Vec<Action>> {
  let path = PathBuf::from("./src/day_2/input.txt");
  let buffer = read_file_to_buffer(path)?;
  let data = buffer
    .lines()
    .map(|x| -> Action {
      let x = x.unwrap();
      let value: Vec<&str> = x.split_whitespace().collect();
      let mouvement = Mouvements::from_str(value[0]).unwrap();
      let amount = value[1].parse::<u32>().unwrap();

      return Action {
        mouvement: mouvement,
        amount: amount,
      };
    })
    .collect();

  return Ok(data);
}

fn solution_2_part_1() -> anyhow::Result<u32> {
  let data = retrieve_data()?;

  let mut horizontal_position = 0;
  let mut depth = 0;
  for action in data {
    match action.mouvement {
      Mouvements::Down => depth += action.amount,
      Mouvements::Up => depth -= action.amount,
      Mouvements::Forward => horizontal_position += action.amount,
    }
  }

  let answer = horizontal_position * depth;

  return Ok(answer);
}

fn solution_2_part_2() -> anyhow::Result<u32> {
  let data = retrieve_data()?;

  let mut aim = 0;
  let mut horizontal_position = 0;
  let mut depth = 0;
  for action in data {
    match action.mouvement {
      Mouvements::Down => aim += action.amount,
      Mouvements::Up => aim -= action.amount,
      Mouvements::Forward => {
        horizontal_position += action.amount;
        depth += aim * action.amount
      }
    }
  }

  let answer = horizontal_position * depth;

  return Ok(answer);
}

pub fn solution_2() -> anyhow::Result<()> {
  let answer1 = solution_2_part_1()?;
  let answer2 = solution_2_part_2()?;
  println!(
    "Solution 2, Answers = {{ (Part 1): {} ; (Part 2): {} }}",
    answer1, answer2
  );
  return Ok(());
}
