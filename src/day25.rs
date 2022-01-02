use crate::utils::read_input;

fn move_direction(direction: i32, mapping: &mut Vec<Vec<i32>>) -> i32 {
    let mut moves = Vec::new();
    for i in 0..mapping.len() {
        for j in 0..mapping[0].len() {
            if mapping[i][j] == direction {
                if direction == 1 {
                    if j == mapping[0].len() - 1 {
                        if mapping[i][0] == 0 {
                            moves.push(((i, j), (i, 0)));
                        }
                    } else {
                        if mapping[i][j + 1] == 0 {
                            moves.push(((i, j), (i, j + 1)));
                        }
                    }
                } else if direction == 2 {
                    if i == mapping.len() - 1 {
                        if mapping[0][j] == 0 {
                            moves.push(((i, j), (0, j)));
                        }
                    } else {
                        if mapping[i + 1][j] == 0 {
                            moves.push(((i, j), (i + 1, j)));
                        }
                    }
                }
            }
        }
    }

    let moves_len = moves.len() as i32;

    for ((i, j), (ii, jj)) in moves {
        mapping[ii][jj] = direction;
        mapping[i][j] = 0;
    }

    moves_len
}

pub fn day25() {
    let input = read_input("./day25/input");

    let height = input.len();
    let width = input[0].len();

    let mut mapping = Vec::new();

    for i in 0..height {
        mapping.push(Vec::new());
        for j in 0..width {
            let val = match input[i].chars().nth(j).unwrap() {
                '.' => 0,
                '>' => 1,
                'v' => 2,
                _ => unreachable!(),
            };
            mapping[i].push(val);
        }
    }

    let mut steps = 0;
    loop {
        steps += 1;
        let mut moves = 0;

        // first eastside
        moves += move_direction(1, &mut mapping);
        moves += move_direction(2, &mut mapping);

        if moves == 0 {
            break;
        }
    }

    println!("{}", steps);
}
