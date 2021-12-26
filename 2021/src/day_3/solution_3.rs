use std::{io::BufRead, path::PathBuf};

use anyhow::{Error, Ok};

use crate::file_manager::read_file_to_buffer;

fn retrieve_data() -> anyhow::Result<Vec<String>> {
  let path = PathBuf::from("./src/day_3/input.txt");
  let buffer = read_file_to_buffer(path)?;
  let data = buffer.lines().map(|x| x.unwrap()).collect();

  return Ok(data);
}

fn solution_3_part1() -> anyhow::Result<usize, Error> {
  let data = retrieve_data()?;
  let mut common_bits: [i32; 12] = [0; 12];

  // Compute common_bits
  for bit_num in &data {
    for (index, bit) in bit_num.chars().enumerate() {
      match bit {
        '0' => common_bits[index] -= 1,
        '1' => common_bits[index] += 1,
        _ => return Err(anyhow::anyhow!("Invalid bit !")),
      }
    }
  }

  let mut gamma_rate_bin = String::from("");
  let mut epsilon_rate_bin = String::from("");
  for rate in &common_bits {
    if rate > &0 {
      gamma_rate_bin += "1";
      epsilon_rate_bin += "0";
    } else {
      gamma_rate_bin += "0";
      epsilon_rate_bin += "1";
    }
  }

  let gamma_rate = usize::from_str_radix(&gamma_rate_bin, 2).unwrap();
  let epsilon_rate = usize::from_str_radix(&epsilon_rate_bin, 2).unwrap();

  let result = gamma_rate * epsilon_rate;

  return Ok(result);
}

#[derive(PartialEq)]
enum Criteria {
  MostCommon,
  LeastCommon,
}

fn get_rate_from_criteria(base_num: &Vec<String>, criteria: Criteria) -> anyhow::Result<String> {
  let mut index = 0;
  let mut bit_nums = base_num.to_vec();
  while bit_nums.len() > 1 {
    let mut common_bits: [i32; 12] = [0; 12];

    // Compute common_bits
    for bit_num in &bit_nums {
      for (index, bit) in bit_num.chars().enumerate() {
        match bit {
          '0' => common_bits[index] -= 1,
          '1' => common_bits[index] += 1,
          _ => return Err(anyhow::anyhow!("Invalid bit !")),
        }
      }
    }

    let char_to_be_eql: char;
    match criteria {
      Criteria::MostCommon => char_to_be_eql = if common_bits[index] >= 0 { '1' } else { '0' },
      Criteria::LeastCommon => char_to_be_eql = if common_bits[index] >= 0 { '0' } else { '1' },
    }
    bit_nums = bit_nums
      .drain(..)
      .filter(|value| value.chars().nth(index).unwrap() == char_to_be_eql)
      .collect();
    index += 1;
  }

  let result = bit_nums[0].to_owned();

  return Ok(result);
}

fn solution_3_part_2() -> anyhow::Result<usize> {
  let data = retrieve_data()?;

  let oxygen_rate_bin = get_rate_from_criteria(&data, Criteria::MostCommon)?;
  let co2_rate_bin = get_rate_from_criteria(&data, Criteria::LeastCommon)?;

  let oxygen_rate = usize::from_str_radix(&oxygen_rate_bin, 2).unwrap();
  let co2_rate = usize::from_str_radix(&co2_rate_bin, 2).unwrap();

  let result = oxygen_rate * co2_rate;

  return Ok(result);
}

pub fn solution_3() -> anyhow::Result<()> {
  let answer1 = solution_3_part1()?;
  let answer2 = solution_3_part_2()?;
  println!(
    "Solution 3, Answers = {{ (Part 1): {} ; (Part 2): {} }}",
    answer1, answer2
  );
  return Ok(());
}
