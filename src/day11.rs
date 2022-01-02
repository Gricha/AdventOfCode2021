use crate::utils::read_input;

pub fn day11() {
    let input = read_input("./day11/input");
    let mut res = input
        .into_iter()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|x| (x as i32 - '0' as i32))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    fn increase_energy(res: &mut Vec<Vec<i32>>, i: i32, j: i32) -> bool {
        // did flash
        if i < 0 || i >= 10 || j < 0 || j >= 10 {
            return false;
        }

        res[i as usize][j as usize] += 1;
        return res[i as usize][j as usize] == 10;
    }

    fn flash(res: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        let mut flash_count = 1;

        for (x, y) in [
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j + 1),
            (i + 1, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
        ] {
            if increase_energy(res, x, y) {
                flash_count += flash(res, x, y);
            }
        }

        flash_count
    }

    let mut idx = 0;
    loop {
        idx += 1;
        // for i in 0..10 {
        //   println!("{:?}", res[i]);
        // }
        let mut flash_count = 0;
        for i in 0..10 {
            for j in 0..10 {
                res[i][j] += 1;
                if res[i][j] == 10 {
                    flash_count += flash(&mut res, i as i32, j as i32);
                }
            }
        }

        for i in 0..10 {
            for j in 0..10 {
                if res[i][j] >= 10 {
                    res[i][j] = 0;
                }
            }
        }

        if flash_count == 100 {
            println!("{}", idx);
            break;
        }
    }
}
