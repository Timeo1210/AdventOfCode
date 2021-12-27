use std::{fs, path::PathBuf};

use anyhow::Ok;
use regex;

#[derive(Debug)]
struct Input {
  rng_nums: Vec<u32>,
  bingos: Vec<[[u32; 5]; 5]>,
}

fn retrieve_data() -> anyhow::Result<Input> {
  let path = PathBuf::from("./src/day_4/input.txt");
  let buffer = fs::read_to_string(path)?;
  let re = regex::Regex::new(r"\n\n").unwrap();
  let data: Vec<&str> = re.split(&buffer).collect();
  let rng_nums: Vec<u32> = data[0]
    .split(',')
    .map(|x| x.parse::<u32>().unwrap() + 1)
    .collect();
  let mut bingos: Vec<[[u32; 5]; 5]> = Vec::new(); // array of five
  for i in 1..data.len() {
    let bingo_index = i - 1;
    bingos.push([[0u32; 5]; 5]);
    for (row, line) in data[i].lines().enumerate() {
      for (index, unparsed_num) in line.split_whitespace().enumerate() {
        let num = unparsed_num.parse::<u32>().unwrap() + 1;
        bingos[bingo_index][row][index] = num.to_owned();
      }
    }
  }

  let result = Input {
    rng_nums: rng_nums,
    bingos: bingos,
  };

  Ok(result)
}

fn validate_bingo(bingo: [[u32; 5]; 5]) -> anyhow::Result<bool> {
  let mut colum_count = [0u32; 5];
  for i in 0..bingo.len() {
    let mut row_count = 0;
    for j in 0..bingo[i].len() {
      if bingo[i][j] == 0 {
        row_count += 1;
      }
      if bingo[j][i] == 0 {
        colum_count[i] += 1;
      }
    }
    if row_count >= 5 {
      return Ok(true);
    }
  }
  if colum_count.contains(&5) {
    Ok(true)
  } else {
    Ok(false)
  }
}

fn sum_bingo(bingo: [[u32; 5]; 5]) -> anyhow::Result<u32> {
  let mut result = 0;
  for i in 0..bingo.len() {
    for j in 0..bingo[i].len() {
      if bingo[i][j] > 0 {
        result += bingo[i][j] - 1;
      }
    }
  }

  Ok(result)
}

fn solution_4_part1() -> anyhow::Result<u32> {
  let mut input = retrieve_data()?;

  let mut final_rng_num = 0;
  let mut final_bingo_num = 0;
  'out: for rng_num in input.rng_nums.iter() {
    for bingo_num in 0..input.bingos.len() {
      for row in 0..input.bingos[bingo_num].len() {
        for n_index in 0..input.bingos[bingo_num][row].len() {
          if input.bingos[bingo_num][row][n_index] == *rng_num {
            input.bingos[bingo_num][row][n_index] = 0;
          }
        }
      }
      let is_bingo_valid = validate_bingo(input.bingos[bingo_num])?;
      if is_bingo_valid == true {
        final_rng_num = *rng_num - 1;
        final_bingo_num = bingo_num;
        break 'out;
      }
    }
  }
  let sum = sum_bingo(input.bingos[final_bingo_num])?;

  let result = final_rng_num * sum;
  Ok(result)
}

fn solution_4_part_2() -> anyhow::Result<u32> {
  let mut input = retrieve_data()?;

  let mut final_rng_num = 0;
  'out: for rng_num in input.rng_nums.iter() {
    let mut bingo_num = 0;
    while bingo_num < input.bingos.len() {
      if bingo_num >= input.bingos.len() {
        bingo_num = input.bingos.len() - 1;
      }
      for row in 0..input.bingos[bingo_num].len() {
        for n_index in 0..input.bingos[bingo_num][row].len() {
          if input.bingos[bingo_num][row][n_index] == *rng_num {
            input.bingos[bingo_num][row][n_index] = 0;
          }
        }
      }
      let is_bingo_valid = validate_bingo(input.bingos[bingo_num])?;
      if is_bingo_valid == true {
        if input.bingos.len() == 1 {
          final_rng_num = *rng_num - 1;
          break 'out;
        }
        input.bingos.remove(bingo_num);
        bingo_num = 0;
      } else {
        bingo_num += 1;
      }
    }
  }
  let sum = sum_bingo(input.bingos[0])?;

  let result = final_rng_num * sum;
  Ok(result)
}

pub fn solution_4() -> anyhow::Result<()> {
  let answer1 = solution_4_part1()?;
  let answer2 = solution_4_part_2()?;
  println!(
    "Solution 4, Answers = {{ (Part 1): {} ; (Part 2): {} }}",
    answer1, answer2
  );
  return Ok(());
}
