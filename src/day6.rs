use crate::utils::read_input;

pub fn day6_1() {
    let input = read_input("./day6/input");
    let numbers: Vec<usize> = input[0]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut quantities: [i64; 9] = [0; 9];
    for i in numbers.into_iter() {
        quantities[i] += 1;
    }

    for _ in 0..256 {
        let spawns = quantities[0];
        for i in 0..8 {
            quantities[i] = quantities[i + 1]
        }
        quantities[8] = spawns;
        quantities[6] += spawns;
    }

    let mut sum = 0;
    for i in 0..9 {
        sum += quantities[i];
    }

    println!("{:?}", sum);
}
