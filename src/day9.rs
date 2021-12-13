use crate::utils::read_input;

pub fn day9() {
  // input modified, added 9s around the data
  let input = read_input("./day9/input");
  let x = input.len();
  let y = input[0].len();

  let mut sum = 0;
  let mapping: Vec<Vec<u32>> = input.into_iter().map(|line| -> Vec<u32> { line.chars().into_iter().map(|c| -> u32 {c as u32 - '0' as u32}).collect()}).collect();

  for i in 0..x {
    for j in 0..y {
      let value = mapping[i][j];
      let mut counts = true;
      if i > 0 {
        if mapping[i-1][j] <= value {
          counts = false;
        }
      }
      if j > 0 {
        if mapping[i][j-1] <= value {
          counts = false;
        }
      }
      if i < x - 1 {
        if mapping[i + 1][j] <= value {
          counts = false;
        }
      }
      if j < y - 1 {
        if mapping[i][j + 1] <= value {
          counts = false;
        }
      }
      if counts {
        sum += value + 1;
      }
    }
  }

  println!("{}", sum);

  let mut basins = [[99999999; 102]; 102];

  //assign unique
  for i in 0..102 {
    for j in 0..102 {
      basins[i][j] = (i*102 + j) + 1;
    }
  }

  // resets 9
  for i in 0..102 {
    for j in 0..102 {
      if mapping[i][j] == 9 {
        basins[i][j] = 0;
      }
    }
  }


  fn crawl(basins:&mut [[usize; 102]; 102], i: usize, j: usize) {
    if basins[i][j] == 0 {
      return;
    }

    // checking for equilibrium
    if (basins[i][j] == basins[i - 1][j] || basins[i - 1][j] == 0) &&
       (basins[i][j] == basins[i + 1][j] || basins[i + 1][j] == 0) &&
       (basins[i][j] == basins[i][j - 1] || basins[i][j - 1] == 0) &&
       (basins[i][j] == basins[i][j + 1] || basins[i][j + 1] == 0) {
      return;
    }

    if basins[i+1][j] > 0 {
      if basins[i+1][j] < basins[i][j] {
        basins[i][j] = basins[i+1][j];
      } else if basins[i+1][j] > basins[i][j] {
        crawl(basins, i+1, j);
      }
    }
    if basins[i][j+1] > 0 {
      if basins[i][j+1] < basins[i][j] {
        basins[i][j] = basins[i][j+1];
      } else if basins[i][j+1] > basins[i][j] {
        crawl(basins, i, j+1);
      }
    }
    if basins[i-1][j] > 0 {
      if basins[i-1][j] < basins[i][j] {
        basins[i][j] = basins[i-1][j];
      } else {
        crawl(basins, i-1, j);
      }
    }
    if basins[i][j-1] > 0 {
      if basins[i][j-1] < basins[i][j] {
        basins[i][j] = basins[i][j-1];
      } else {
        crawl(basins, i, j-1);
      }
    }
  }

  for i in 1..102 {
    for j in 1..102 {
      crawl(&mut basins, i, j);
    }
  }

  let mut basin_sizes = Vec::new();
  for _ in 0..(102*102) {
    basin_sizes.push(0);
  }

  for i in 0..102 {
    for j in 0..102 {
      if basins[i][j] > 0 {
        basin_sizes[basins[i][j]] += 1;
      }
    }
  }

  basin_sizes.sort();
  let value = basin_sizes.iter().rev().take(3).fold(1, |sum, i| sum * i);

  println!("{:?}", value);
}