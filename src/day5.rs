use std::cmp::max;
use std::cmp::min;

use crate::utils::read_input;

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

type Line = (Coordinate, Coordinate);

pub fn day5_1() {
    let input = read_input("./day5/input");

    let mut data: Vec<Line> = Vec::new();
    for i in input.into_iter() {
        let components: Vec<Coordinate> = i
            .split("->")
            .map(|part| {
                let stripped: String = part.chars().filter(|c| !c.is_whitespace()).collect();
                stripped
            })
            .map(|x: String| {
                let parts: Vec<i32> = x
                    .split(",")
                    .map(|num_str| num_str.parse::<i32>().unwrap())
                    .collect();
                Coordinate {
                    x: parts[0],
                    y: parts[1],
                }
            })
            .collect();
        data.push((components[0], components[1]));
    }

    let mut plot = [[0; 1000]; 1000];

    for (i, j) in data {
        if i.x == j.x {
            for k in min(i.y, j.y)..=max(i.y, j.y) {
                let x = i.x as usize;
                plot[x][k as usize] += 1;
            }
        } else if i.y == j.y {
            for k in min(i.x, j.x)..=max(i.x, j.x) {
                let y = i.y as usize;
                plot[k as usize][y] += 1;
            }
        } else {
            // By the data format we are guaranteed that these lines
            // will be diagonal under 45 degrees angle
            // One thing it means is that max(x1, x2) - min(x1, x2) is
            // always the length of the line because len(y) == len(x)
            let line_length = max(i.x, j.x) - min(i.x, j.x);
            for offset in 0..=line_length {
                let x_coord = i.x + offset * (j.x - i.x).signum();
                let y_coord = i.y + offset * (j.y - i.y).signum();
                plot[x_coord as usize][y_coord as usize] += 1;
            }
        }
    }

    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if plot[i][j] >= 2 {
                count += 1;
            }
        }
    }

    println!("{:?}", count);
}
