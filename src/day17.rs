use crate::utils::read_input;
use std::cmp::max;

pub fn day17_1() {
    let input = read_input("./day17/input");
    let line = &input[0];

    let (_, _, ymin, _) =
        scan_fmt!(line, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();

    let max_height = ((ymin + 1) * ymin) / 2;
    println!("{}", max_height);
}

pub fn day17() {
    let input = read_input("./day17/input");
    let line = &input[0];

    let (xmin, xmax, ymin, ymax) =
        scan_fmt!(line, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();

    let absolute_max_x = xmax;
    let absolute_max_y = (-ymin) + 1;

    let mut count = 0;
    for i in 0..=absolute_max_x {
        for j in ymin..=absolute_max_y {
            let mut pos = (0, 0);
            let mut velocity = (i, j);
            while pos.0 <= xmax && pos.1 >= ymin {
                if pos.0 >= xmin && pos.1 <= ymax {
                    count += 1;
                    break;
                }
                pos.0 += velocity.0;
                pos.1 += velocity.1;
                velocity.0 = max(0, velocity.0 - 1);
                velocity.1 = velocity.1 - 1;
            }
        }
    }
    println!("{}", count);
}
