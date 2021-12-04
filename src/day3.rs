
use crate::utils::read_input;

fn convert_input(str: &str) -> Vec<i32> {
  str.as_bytes().into_iter().map(|b|  {
      let c = *b as char;
      match c {
          '1' => 1,
          '0' => 0,
          _ => panic!("impossible")
      }
  }).collect()
}

pub fn day3_2() {
  let input = read_input("./day3/input");

  let mut filtered_input = input.clone();
  for i in 0..12 {
      let num_rows = filtered_input.len();
      if num_rows == 1 {
          break;
      }
      let consensus: f64 = num_rows as f64 / 2.0;
      let values = count_bits(&filtered_input);
      let selected_bit: char;
      if values[i] as f64 >= consensus {
          selected_bit = '1';
      } else {
          selected_bit = '0';
      }
      filtered_input = filtered_input.into_iter().filter(|x| {
          x.as_bytes()[i] as char == selected_bit
      }).collect();
  }

  let mut filtered_input_secondary = input;
  for i in 0..12 {
      let num_rows = filtered_input_secondary.len();
      if num_rows == 1 {
          break;
      }
      let consensus: f64 = num_rows as f64 / 2.0;
      let values = count_bits(&filtered_input_secondary);
      let selected_bit: char;
      if (values[i] as f64) < consensus {
          selected_bit = '1';
      } else {
          selected_bit = '0';
      }
      filtered_input_secondary = filtered_input_secondary.into_iter().filter(|x| {
          x.as_bytes()[i] as char == selected_bit
      }).collect();
  }

  let oxygen_bytes = convert_input(&filtered_input[0]);
  let co2_bytes = convert_input(&filtered_input_secondary[0]);

  let mut oxygen_generator_rating = 0;
  let mut co2_scrubber_rating = 0;
  let mut mul = 1;
  for i in (0..12).rev() {
      oxygen_generator_rating += mul * oxygen_bytes[i];
      co2_scrubber_rating += mul * co2_bytes[i];
      mul *= 2;
  }
  println!("{:?}", co2_scrubber_rating*&oxygen_generator_rating);
}

fn count_bits(input: &Vec<String>) -> Vec<i32> {
  let mut values: Vec<i32> = vec![0; 12];

  for l in input.iter() {
      let mut idx = 0;
      let lb = l.as_bytes();
      while idx < 12 {
          if lb[idx] as char == '1' {
              values[idx] += 1;
          }
          idx += 1
      }
  }
  values
}

pub fn day3_1() {
  let input = read_input("./day3/input");

  let values: Vec<i32> = count_bits(&input);

  let bits: Vec<i32> = values.into_iter().map(|x| { if x > 500 { 1} else {0}}).collect();

  let mut gamma = 0;
  let mut epsilon = 0;
  let mut mul = 1;
  for i in (0..12).rev() {
      gamma += mul * bits[i];
      epsilon += mul * (1 - bits[i]);
      mul *= 2;
  }

  println!("{:?}", gamma * epsilon);
}
