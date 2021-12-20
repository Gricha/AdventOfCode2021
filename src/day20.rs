use crate::utils::read_input;
use itertools::Itertools;

fn bin_to_usize(slice: &[bool]) -> usize {
  let mut val = 0;
  slice.iter().for_each(|bit| {
    val *= 2;
    val += *bit as usize;
  });
  val
}

pub fn day20() {
  let input = read_input("./day20/input");
  let byte_mapping = input[0].chars().into_iter().map(|x| {
    match x {
      '#' => true,
      '.' => false,
      _ => unreachable!(),
    }
  }).collect_vec();

  // Map is 100x100 at first
  // it shouldn't grow more than 50 each direction after step 35 so let's assume 200
  let initial_bytes = input.iter().skip(2).map(|x| x.chars().map(|c| {
    match c {
      '#' => true,
      '.' => false,
      _ => unreachable!(),
    }
  }).collect_vec()).collect_vec();

  let mut data = [[false; 400]; 400];
  for i in 0..100 {
    for j in 0..100 {
      data[i+150][j+150] = initial_bytes[i][j];
    }
  }

  for _ in 0..50 {
    let mut computational = [[false; 400]; 400];
    for i in 0..400 {
      for j in 0..400 {
        if i == 0 || i == 399 || j == 0 || j == 399 {
          // We're on the border, given input data, we're just flipping the bit
          computational[i][j] = !data[i][j];
          continue;
        }

        let binary = [
          data[i-1][j-1], 
          data[i-1][j], 
          data[i-1][j+1], 
          data[i][j-1], 
          data[i][j], 
          data[i][j+1],
          data[i+1][j-1], 
          data[i+1][j], 
          data[i+1][j+1], 
        ];
        let val = byte_mapping[bin_to_usize(&binary)];
        computational[i][j] = val;
      }
    }
    data = computational;
  }

  let mut count = 0;
  for i in 0..400 {
    for j in 0..400 {
      count += data[i][j] as i32;
    }
  }
  println!("{}", count);
}