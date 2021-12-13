use std::collections::{HashSet, HashMap};

use crate::utils::read_input;

enum PatternMatch {
  Correct(Vec<char>),
  Incorrect(char)
}

pub fn day10() {
  let input = read_input("./day10/input");

  let openers= HashSet::from(['(', '[', '<', '{']);
  let mut pairings = HashMap::new();
  pairings.insert('(', ')');
  pairings.insert('[', ']');
  pairings.insert('{', '}');
  pairings.insert('<', '>');

  let mut res= input.into_iter()
  .filter_map(|l| {
    match l.chars()
        .into_iter()
        .fold(PatternMatch::Correct(Vec::new()), |pattern_stack, value| {
          match pattern_stack {
            PatternMatch::Incorrect(val) => {
              PatternMatch::Incorrect(val)
            },
            PatternMatch::Correct(mut stack) => {
              if openers.contains(&value) {
                stack.push(value);
                PatternMatch::Correct(stack)
              } else {
                match stack.last() {
                  Some(last_value) => {
                    if pairings.get(last_value).unwrap() == &value {
                      stack.pop();
                      PatternMatch::Correct(stack)
                    } else {
                      PatternMatch::Incorrect(value)
                    }
                  },
                  None => {
                    PatternMatch::Incorrect(value)
                  }
                }
              }
            }
          }
        }) {
        // PatternMatch::Incorrect(val) => Some(val), // from part 1
        // PatternMatch::Correct(_) => None
        PatternMatch::Incorrect(_) => None,
        PatternMatch::Correct(stack) => Some(stack) 
    }
  })
  .map(|x| {
    x.into_iter().rev().map(|c| {
      pairings[&c]
    })
    .fold(0, |mut acc, val| {
      acc *= 5;
      acc += match val {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!()
      };
      acc
    })
  }).collect::<Vec<i64>>();
  // .map(|x| match x { // from part 1
  //   ')' => 3,
  //   ']' => 57,
  //   '}' => 1197,
  //   '>' => 25137,
  //   _ => unreachable!()
  // }).sum();
  res.sort();
  let size = res.len();
  let mid = (size - 1) / 2;
  println!("{}", res[mid]);
}