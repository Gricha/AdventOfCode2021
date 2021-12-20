use crate::utils::read_input;
use itertools::Itertools;


#[derive(Debug, Clone)]
enum Number {
  Literal(i32),
  Nested(Box<InnerNumber>),
}

#[derive(Debug, Clone)]
struct InnerNumber {
  left:  Box<Number>,
  right: Box<Number>,
}

fn print_number(number_box: &Box<Number>) {
  inner_print_number(number_box);
  println!();
}

fn inner_print_number(number_box: &Box<Number>) {
  match number_box.as_ref() {
    Number::Literal(v) => print!("{}", v),
    Number::Nested(i) => {
      print!("[");
      inner_print_number(&i.left);
      print!(",");
      inner_print_number(&i.right);
      print!("]")
    },
  }
}

fn consume_value_right(number_box: &mut Box<Number>, value: i32) -> i32 {
  let number = number_box.as_mut();
  match number {
    Number::Nested(inner) => {
      consume_value_right(&mut inner.left, value);
      0
    }
    Number::Literal(val) => {
      *val += value;
      0
    }
  }
}

fn consume_value_left(number_box: &mut Box<Number>, value: i32) -> i32 {
  let number = number_box.as_mut();
  match number {
    Number::Nested(inner) => {
      consume_value_left(&mut inner.right, value);
      0
    }
    Number::Literal(val) => {
      *val += value;
      0
    }
  }
}

fn split_number(number_box: &mut Box<Number>) -> bool {
  let number = number_box.as_mut();
  match number {
    Number::Literal(val) => {
      if *val >= 10 {
        let left = *val / 2;
        let right = (*val + 1) / 2;
        std::mem::replace(number, Number::Nested(Box::new(InnerNumber {
          left: Box::new(Number::Literal(left)),
          right: Box::new(Number::Literal(right)),
        })));
        true
      } else {
        false
      }
    },
    Number::Nested(inner) => {
      let splitted = split_number(&mut inner.left);
      if splitted {
        return true;
      } else {
        return split_number(&mut inner.right);
      }
    },
  }
}

fn reduce_number_with_explosion_inner(number_box: &mut Box<Number>, depth: i32) -> (bool, (i32, i32)) {
  let number = number_box.as_mut();
  match number {
    Number::Nested(inner) => {
      if depth == 4 {
        // explode
        let left = match *inner.left {
          Number::Literal(l) => l,
          _ => unreachable!(),
        };
        let right = match *inner.right {
          Number::Literal(l) => l,
          _ => unreachable!(),
        };
        std::mem::replace(number, Number::Literal(0));
        return (true, (left, right));
      } else {
        let (exploded, (l, r)) = reduce_number_with_explosion_inner(&mut inner.left, depth + 1);
        if exploded {
          return (true, (l, consume_value_right(&mut inner.right, r)));
        }

        let (exploded, (l, r)) = reduce_number_with_explosion_inner(&mut inner.right, depth + 1);
        if exploded {
          return (true, (consume_value_left(&mut inner.left, l), r));
        }

        return (false, (0, 0));
      }
    },
    _ => {}
  }
  (false, (0,0))
}

fn parse_number(l: &str, i: usize) -> (usize, Number) {
  let mut idx = i;

  if l.chars().nth(idx).unwrap() == '[' {
    // We got nested
    idx += 1;
    let (new_idx, left) = parse_number(l, idx);
    idx = new_idx;
    idx += 1; // comma
    let (new_idx, right) = parse_number(l, idx);
    idx = new_idx;
    idx += 1; // ]
    (idx, Number::Nested(Box::new(InnerNumber {
      left: Box::new(left),
      right: Box::new(right),
    })))

  } else {
    let num = l.chars().nth(idx).unwrap() as i32 - '0' as i32;
    idx += 1;
    (idx, Number::Literal(num))
  }
}

fn cleanup_number(number_box: &mut Box<Number>) {
  loop {
    let (expl, _) = reduce_number_with_explosion_inner(number_box, 0);
    if expl {
      continue;
    } else {
      let splitted = split_number(number_box);
      if !splitted {
        break;
      }
    }
  }
}

fn add_numbers(number_1: Box<Number>, number_2: Box<Number>) -> Box<Number> {
  let mut n = Box::new(Number::Nested(Box::new(InnerNumber {
    left: number_1,
    right: number_2,
  })));

  cleanup_number(&mut n);

  n
}

fn magnitude(number: &Box<Number>) -> i32 {
  match number.as_ref() {
    Number::Literal(v) => *v,
    Number::Nested(i) => {
      (magnitude(&i.left) * 3) + (magnitude(&i.right) * 2)
    }
  }
}

pub fn day18_1() {
  let input = read_input("./day18/input");
  let numbers = input.into_iter().map(|line| {
    Box::new(parse_number(&line, 0).1)
  }).collect_vec();

  let mut carry: Option<Box<Number>> = None;
  for i in numbers {
    match carry {
      None => {
        carry = Some(i);
      },
      Some(c) => {
        carry = Some(add_numbers(c, i));
      }
    }
  }
  print_number(carry.as_ref().unwrap());
  println!("{}", magnitude(&carry.unwrap()));
}

pub fn day18() {
  let input = read_input("./day18/input");
  let numbers = input.into_iter().map(|line| {
    Box::new(parse_number(&line, 0).1)
  }).collect_vec();

  let mut max = 0;
  for i in 0..numbers.len() {
    for j in 0..numbers.len() {
      if i == j {
        continue;
      }

      let magn = magnitude(&add_numbers(numbers[i].clone(), numbers[j].clone()));
      max = std::cmp::max(max, magn);
    }
  }

  println!("{}", max);
}