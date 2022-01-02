use std::collections::BTreeSet;

use crate::utils::read_input;
use itertools::Itertools;

pub fn day13() {
    let input = read_input("./day13/input");

    let dots = input
        .iter()
        .filter(|x| !x.starts_with("fold"))
        .map(|l| {
            let (x_str, y_str): (&str, &str) = l.split(",").collect_tuple().unwrap();
            (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap())
        })
        .collect::<Vec<(i32, i32)>>();
    let instructions = input
        .iter()
        .filter(|x| x.starts_with("fold"))
        .map(|l| {
            let trimmed = l.strip_prefix("fold along ").unwrap();
            let (direction_str, coordinate_str): (&str, &str) =
                trimmed.split("=").collect_tuple().unwrap();
            assert!(direction_str.len() == 1);
            let dir = direction_str.chars().nth(0).unwrap();
            let coordinate = coordinate_str.parse::<i32>().unwrap();
            (dir, coordinate)
        })
        .collect::<Vec<(char, i32)>>();

    // single foldline

    let mut current_dots = dots;
    for foldline in instructions.into_iter() {
        let mut dot_set = BTreeSet::new();
        for dot in current_dots.iter() {
            if foldline.0 == 'x' {
                assert!(dot.0 != foldline.1);
                if dot.0 < foldline.1 {
                    dot_set.insert(dot.clone());
                } else {
                    let distance = dot.0 - foldline.1;
                    dot_set.insert((foldline.1 - distance, dot.1));
                }
            } else {
                assert!(dot.1 != foldline.1);
                if dot.1 < foldline.1 {
                    dot_set.insert(dot.clone());
                } else {
                    let distance = dot.1 - foldline.1;
                    dot_set.insert((dot.0, foldline.1 - distance));
                }
            }
        }
        current_dots = dot_set.into_iter().collect();
    }

    let mut grid = [[0; 40]; 40];

    for (x, y) in current_dots.into_iter() {
        grid[x as usize][y as usize] = 1;
    }

    for i in 0..40 {
        for j in 0..40 {
            if grid[j][i] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
