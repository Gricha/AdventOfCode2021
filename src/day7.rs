use std::{i32::MAX};

use crate::utils::read_input;
use std::cmp::min;

pub fn day7_1() {
  let input = read_input("./day7/input");
  let numbers: Vec<i32> = input[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();

  let mut cost = MAX;
  for i in 0..2001 {
    let mut current_cost = 0;
    for k in numbers.iter() {
      let distance = (k-i).abs();
      let step_cost = ((distance + 1) * distance) / 2;

      current_cost += step_cost;
    }

    cost = min(cost, current_cost);
  }

  println!("{:?}", cost);
}