use crate::utils::read_numerical_input;

pub fn day1() {
    let input = read_numerical_input("./day1/input");

    let mut count = 0;
    let mut i = 3;

    while i < input.len() {
        if input[i] > input[i - 3] {
            count += 1
        }

        i += 1
    }

    println!("{:?}", count);
}
