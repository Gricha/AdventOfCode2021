use crate::utils::read_input;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Command {
    direction: Direction,
    value: i32,
}

pub fn day2() {
    let input = read_input("./day2/input");

    let mut commands = Vec::new();
    for v in input {
        let components: Vec<&str> = v.split(" ").collect();
        let value = components[1].parse::<i32>().unwrap();
        let direction = match components[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("yikes"),
        };
        commands.push(Command { direction, value })
    }

    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for c in commands {
        match c.direction {
            Direction::Up => {
                aim -= c.value;
            }
            Direction::Down => {
                aim += c.value;
            }
            Direction::Forward => {
                horizontal += c.value;
                vertical += aim * c.value;
            }
        }
    }

    println!("{:?}", horizontal * vertical);
}
