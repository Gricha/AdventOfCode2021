use crate::utils::read_input;

const BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct BingoBoard {
  numbers: [[i32; BOARD_SIZE]; BOARD_SIZE],
  markers: [[i32; BOARD_SIZE]; BOARD_SIZE],
  called_bingo: bool,
}

fn parse_line(line: &str) -> [i32; BOARD_SIZE] {
  let components: Vec<&str> = line.split(" ").filter(|x| !x.is_empty()).collect();
  let mut values = [0,0,0,0,0];
  for i in 0..BOARD_SIZE {
    let value = components[i].parse::<i32>().unwrap();
    values[i] = value;
  }
  values
}

impl BingoBoard {
  fn create_with_input(input: Vec<&str>) -> Self {
    let mut numbers = [[0;BOARD_SIZE];BOARD_SIZE];
    let markers = [[0;BOARD_SIZE];BOARD_SIZE];

    for i in 0..BOARD_SIZE {
      let line = input.get(i).expect("INCORRECT INPUT");
      let line_numbers = parse_line(line);
      numbers[i] = line_numbers;
    }

    BingoBoard {
      numbers,
      markers,
      called_bingo: false
    }
  }

  fn mark(&mut self, number: i32) {
    for i in 0..BOARD_SIZE {
      for j in 0..BOARD_SIZE {
        if self.numbers[i][j] == number {
          self.markers[i][j] = 1;
        }
      }
    }
  }

  fn call_bingo(&mut self) -> bool {
    if self.called_bingo {
      return false;
    }
    self.called_bingo = true;
    true
  }

  fn check_if_bingo(&self) -> bool {
    for i in 0..BOARD_SIZE {
      let mut failed = false;
      for j in 0..BOARD_SIZE {
        if self.markers[i][j] != 1 {
          failed = true;
          break
        }
      }

      if !failed {
        return true;
      }
    }

    for i in 0..BOARD_SIZE {
      let mut failed = false;
      for j in 0..BOARD_SIZE {
        if self.markers[j][i] != 1 {
          failed = true;
          break
        }
      }

      if !failed {
        return true;
      }
    }

    false
  }
}

pub fn day4_1() {
  let input = read_input("./day4/input");
  let moves_str = &input[0];

  let mut boards_str: Vec<Vec<&str>> = Vec::new();
  for i in 1..input.len() {
    if input[i].is_empty() {
      boards_str.push(Vec::new());
    } else {
      boards_str.last_mut().expect("needs value").push(&input[i]);
    }
  }

  let mut boards: Vec<BingoBoard> = Vec::new();
  for b in boards_str.into_iter() {
    let board = BingoBoard::create_with_input(b);
    boards.push(board);
  }

  let moves: Vec<i32> = moves_str.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
  let mut boards_that_won = 0;
  let number_of_boards = boards.len();
  for m in moves {
    for b in boards.iter_mut() {
      b.mark(m);

      if b.check_if_bingo() {
        if b.call_bingo() {
          boards_that_won += 1;
          if boards_that_won == number_of_boards {
            let mut sum_unmarked = 0;
            for i in 0..BOARD_SIZE {
              for j in 0..BOARD_SIZE {
                if b.markers[i][j] == 0 {
                  sum_unmarked += b.numbers[i][j];
                }
              }
            }
            println!("{:?}", m * sum_unmarked);
            return;
          }
        }
      }
    }
  }
}