use std::cmp::Ordering;
use std::cmp::{min, max};
use std::collections::{BinaryHeap, HashSet};

fn get_weight(c: &char) -> i32 {
  match c {
    'A' => 1,
    'B' => 10,
    'C' => 100,
    'D' => 1000,
    _ => unreachable!()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Castle {
  corridor: [char; 11],
  rooms: [[char; 4]; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
  castle: Castle,
  cost: i32,
}

impl Castle {
  fn check_room(&self, idx: usize) -> bool {
    for c in self.rooms[idx].iter() {
      if *c == '.' {
        return false;
      }
      if *c as usize - 'A' as usize != idx {
        return false;
      }
    }
    return true;
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      other.cost.partial_cmp(&self.cost)
  }
}

impl Ord for State {
  fn cmp(&self, other: &State) -> Ordering {
      self.partial_cmp(other).unwrap()
  }
}

pub fn day23() {
  let input = Castle {
    corridor: ['.'; 11],
    rooms: [
      ['D', 'D', 'D', 'C'],
      ['B', 'C', 'B', 'C'],
      ['D', 'B', 'A', 'A'],
      ['A', 'A', 'C', 'B'],
    ]
  };

  let desired_castle = Castle {
    corridor: ['.'; 11],
    rooms: [
      ['A'; 4],
      ['B'; 4],
      ['C'; 4],
      ['D'; 4],
    ],
  };

  let initial_state = State {
    castle: input,
    cost: 0,
  };

  let mut queue = BinaryHeap::new();
  queue.push(initial_state);
  let mut visited = HashSet::new();

  let mut cost = 0;
  while let Some(state) = queue.pop() {
    if state.castle == desired_castle {
      cost = state.cost;
      break;
    }

    // Check if node has been visited
    if visited.contains(&state.castle) {
      continue;
    }

    // Now the actual moves can be done

    // This is doing a moves from corridor directly to the rooms
    for i in 0..11 {
      if i == 2 || i == 4 || i == 6 || i == 8 {
        continue;
      }
      let c = state.castle.corridor[i];
      if c == '.' {
        continue;
      }

      let target_room_index = c as i32 - 'A' as i32;

      let target_room_position = (target_room_index + 1) * 2;
      let distance = max(i as i32, target_room_position) - min(i as i32, target_room_position);

      let mut can_walk = true;
      for j in min(i as i32, target_room_position)..=max(i as i32, target_room_position) {
        if j == i as i32 {
          // the same character is expected
          continue;
        }
        if state.castle.corridor[j as usize] != '.' {
          can_walk = false;
          break;
        }
      }
      for r in state.castle.rooms[target_room_index as usize].iter() {
        if *r != '.' && *r != c {
          can_walk = false;
          break;
        }
      }
      if !can_walk {
        continue;
      }

      // Now we assume we kinda can move there. There are two positions in the room but we can only take
      // one - we either move to the back, or there is already a letter in the back.
      let mut steps: i32 = 0;
      let mut new_castle = state.castle.clone();

      for k in (0..4).rev() {
        if new_castle.rooms[target_room_index as usize][k] == '.' {
          new_castle.rooms[target_room_index as usize][k] = c;
          steps = k as i32 + 1;
          break;
        }
      }
      new_castle.corridor[i] = '.';
      let cost = (distance + steps) * get_weight(&c);
      queue.push(State {
        castle: new_castle,
        cost: cost + state.cost
      })
    }

    // This is quitting the rooms
    for (room_idx, room) in state.castle.rooms.iter().enumerate() {
      if state.castle.check_room(room_idx) {
        // room has been set
        continue;
      }
      
      for (idx, c) in room.iter().enumerate() {
        // for 1st one we can just generate moves to all positions in the corridor
        // that are legal.
        // for 2nd one we do the same but cost is +1 and can only occur if idx[0] == '.'

        if *c == '.' {
          continue;
        }

        let mut needs_hop = false;
        for k in 0..idx {
          if state.castle.rooms[room_idx][k] != '.' {
            needs_hop = true;
            break;
          }
        }
        if needs_hop {
          continue;
        }

        // All possible moves can happen to each position of of the corridor
        for i in 0..11 {
          if (i == 2) || (i == 4) || (i == 6) || (i == 8) {
            continue;
          }

          // There are 7 corridor positions
          // The position needs to be empty
          // Every spot on the path needs to be empty

          // Rooms are at indexes: 2,4,6,8
          let corridor_room_index = (room_idx + 1) * 2;
          
          let mut can_walk = true;
          // Validate that path is clear
          for j in min(i, corridor_room_index)..=max(i, corridor_room_index) {
            if state.castle.corridor[j] != '.' {
              can_walk = false;
              break;
            }
          }
          if !can_walk {
            // This state will be impossible
            continue;
          }

          // Otherwise we know we can walk to this position from this room hence we should
          // do so and add the cost up
          let cost = ((max(i, corridor_room_index) - min(i, corridor_room_index)) + idx + 1) as i32 * get_weight(&c);
          let mut new_castle = state.castle.clone();
          new_castle.rooms[room_idx][idx] = '.';
          new_castle.corridor[i] = *c;
          queue.push(State {
            cost: cost + state.cost,
            castle: new_castle,
          })
        }
      }
    }


    visited.insert(state.castle);
  }

  println!("{}", cost);
}