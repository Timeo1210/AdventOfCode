use std::{collections::HashMap, fs, path::PathBuf};

fn retrieve_data() -> anyhow::Result<Vec<u32>> {
  let path = PathBuf::from("./src/day_6/input.txt");
  let buffer = fs::read_to_string(path)?;
  let data = buffer
    .split(',')
    .map(|x| x.parse::<u32>().unwrap())
    .collect();

  Ok(data)
}

fn solution_6_part1() -> anyhow::Result<u32> {
  let mut data = retrieve_data()?;
  let days_count: u32 = 80;

  let mut birth_count = 0;
  for _ in 0..days_count {
    for index in 0..data.len() {
      if data[index] == 0 {
        data[index] = 6;
        birth_count += 1;
      } else {
        data[index] -= 1;
      }
    }

    for _ in 0..birth_count {
      data.push(8);
    }
    birth_count = 0;
  }

  let result = data.len();

  Ok(result as u32)
}

fn solution_6_part_2() -> anyhow::Result<u64> {
  let data = retrieve_data()?;
  let days_count = 256;

  let mut fish_hash: HashMap<u64, u64> = HashMap::new();
  for i in 0..9 {
    fish_hash.insert(i, 0);
  }
  for fish in &data {
    *fish_hash.entry((*fish).into()).or_insert(0) += 1;
  }

  for _ in 0..days_count {
    let zeroes = *fish_hash.entry(0).or_insert(0);
    *fish_hash.entry(0).or_insert(0) = 0;
    for index in 1..9 {
      *fish_hash.entry(index - 1).or_insert(0) += *fish_hash.entry(index).or_insert(0);
      *fish_hash.entry(index).or_insert(0) = 0;
    }
    *fish_hash.entry(6).or_insert(0) += zeroes;
    *fish_hash.entry(8).or_insert(0) += zeroes;
  }

  let mut result = 0;
  for index in 0..9 {
    result += *fish_hash.entry(index).or_insert(0);
  }

  Ok(result)
}

pub fn solution_6() -> anyhow::Result<()> {
  let answer1 = solution_6_part1()?;
  let answer2 = solution_6_part_2()?;
  println!(
    "Solution 6, Answers = {{ (Part 1): {} ; (Part 2): {} }}",
    answer1, answer2
  );
  return Ok(());
}
