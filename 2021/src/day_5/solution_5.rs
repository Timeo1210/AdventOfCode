use std::{fs, path::PathBuf};

#[derive(Debug)]
enum Direction {
  Horizontal,
  Vertical,
  Diagonal,
}

#[derive(Debug)]
struct Coord {
  r: i32,
  c: i32,
}
impl Default for Coord {
  fn default() -> Coord {
    Coord { r: 0, c: 0 }
  }
}

#[derive(Debug)]
struct Wind {
  from: Coord,
  to: Coord,
  d: Direction,
}
impl Default for Wind {
  fn default() -> Wind {
    Wind {
      from: Coord::default(),
      to: Coord::default(),
      d: Direction::Horizontal,
    }
  }
}

fn retrieve_data(with_diagonal: bool) -> anyhow::Result<Vec<Wind>> {
  let path = PathBuf::from("./src/day_5/input.txt");
  let buffer = fs::read_to_string(path)?;
  let mut data: Vec<Wind> = Vec::new();
  let re = regex::Regex::new(r"->").unwrap();
  for line in buffer.lines() {
    let str_winds: Vec<&str> = re.split(line).collect();
    let mut wind: Wind = Wind::default();
    for wind_index in 0..str_winds.len() {
      let str_wind: Vec<&str> = str_winds[wind_index].split(',').collect();
      let coord = Coord {
        c: str_wind[0].trim().parse::<i32>().unwrap(),
        r: str_wind[1].trim().parse::<i32>().unwrap(),
      };
      match wind_index {
        0 => wind.from = coord,
        1 => wind.to = coord,
        _ => return Err(anyhow::anyhow!("Invalid wind_index")),
      }
    }
    if wind.from.r == wind.to.r {
      wind.d = Direction::Horizontal;
      data.push(wind);
    } else if wind.from.c == wind.to.c {
      wind.d = Direction::Vertical;
      data.push(wind);
    } else if with_diagonal == true {
      wind.d = Direction::Diagonal;
      data.push(wind)
    }
  }

  Ok(data)
}

fn solution_5_part1() -> anyhow::Result<u32> {
  let data = retrieve_data(false)?;
  let mut matrice = [[0u32; 1000]; 1000];

  // Place Wind
  for wind in data {
    match wind.d {
      Direction::Horizontal => {
        let diff = wind.to.c - wind.from.c;
        let start_index = u32::try_from(std::cmp::min(wind.from.c, wind.to.c))?;
        let length = u32::try_from(i32::abs(diff))?;
        let end_index = start_index + length + 1;
        for index in start_index..end_index {
          matrice[wind.from.r as usize][index as usize] += 1;
        }
      }
      Direction::Vertical => {
        // let diff = i32::try_from(wind.to.r - wind.from.r)?;
        let diff: i32 = wind.to.r - wind.from.r;
        let start_index = u32::try_from(std::cmp::min(wind.from.r, wind.to.r))?;
        let length = u32::try_from(i32::abs(diff))?;
        let end_index = start_index + length + 1;
        for row in start_index..end_index {
          matrice[row as usize][wind.from.c as usize] += 1;
        }
      }
      Direction::Diagonal => {}
    }
  }

  // Count overlap
  let mut count = 0;
  for (row, _) in matrice.iter().enumerate() {
    for index in 0..999 {
      if matrice[row][index] >= 2 {
        count += 1;
      }
    }
  }

  Ok(count)
}

fn solution_5_part_2() -> anyhow::Result<u32> {
  let data = retrieve_data(true)?;
  let mut matrice = [[0u32; 1000]; 1000];

  // Place Wind
  for wind in data {
    match wind.d {
      Direction::Horizontal => {
        let diff = wind.to.c - wind.from.c;
        let start_index = u32::try_from(std::cmp::min(wind.from.c, wind.to.c))?;
        let length = u32::try_from(i32::abs(diff))?;
        let end_index = start_index + length + 1;
        for index in start_index..end_index {
          matrice[wind.from.r as usize][index as usize] += 1;
        }
      }
      Direction::Vertical => {
        // let diff = i32::try_from(wind.to.r - wind.from.r)?;
        let diff: i32 = wind.to.r - wind.from.r;
        let start_index = u32::try_from(std::cmp::min(wind.from.r, wind.to.r))?;
        let length = u32::try_from(i32::abs(diff))?;
        let end_index = start_index + length + 1;
        for row in start_index..end_index {
          matrice[row as usize][wind.from.c as usize] += 1;
        }
      }
      Direction::Diagonal => {
        let vertical_dir = if wind.from.r - wind.to.r < 0 { 1 } else { -1 };
        let horizontal_dir = if wind.from.c - wind.to.c < 0 { 1 } else { -1 };
        let mut vertical_start_index = wind.from.r;
        let mut horizontal_start_index = wind.from.c;
        while vertical_start_index != wind.to.r + vertical_dir
          && horizontal_start_index != wind.to.c + horizontal_dir
        {
          matrice[vertical_start_index as usize][horizontal_start_index as usize] += 1;
          vertical_start_index += vertical_dir;
          horizontal_start_index += horizontal_dir;
        }
      }
    }
  }

  // Count overlap
  let mut count = 0;
  for (row, _) in matrice.iter().enumerate() {
    for index in 0..999 {
      if matrice[row][index] >= 2 {
        count += 1;
      }
    }
  }

  Ok(count)
}

pub fn solution_5() -> anyhow::Result<()> {
  let answer1 = solution_5_part1()?;
  let answer2 = solution_5_part_2()?;
  println!(
    "Solution 5, Answers = {{ (Part 1): {} ; (Part 2): {} }}",
    answer1, answer2
  );
  return Ok(());
}
