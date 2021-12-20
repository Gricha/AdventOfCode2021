use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_input;

const REMAP: [(i32, i32, i32); 6] = [(0, 1, 2), (0, 2, 1), (1, 0, 2), (1, 2, 0), (2, 0, 1), (2, 1, 0)];
const TRANSLATE: [(i32, i32, i32); 8] = [(1,1,1), (1, 1, -1), (1, -1, 1), (-1, 1, 1), (1, -1, -1), (-1, 1, -1), (-1, -1, 1), (-1, -1, -1)];

fn translate(p: &(i32, i32, i32), rotation:&(&(i32, i32, i32), &(i32, i32, i32))) -> (i32, i32, i32) {
  let pa= [p.0, p.1, p.2];
  (rotation.1.0 * pa[rotation.0.0 as usize], rotation.1.1 * pa[rotation.0.1 as usize], rotation.1.2 * pa[rotation.0.2 as usize])
}

fn load_data(input: Vec<String>) -> Vec<Vec<(i32, i32, i32)>> {
  let mut scanners_data = Vec::new();
  let mut current_scanner = Vec::new();

  for i in input {
    if i.starts_with("--- scanner") {
      scanners_data.push(current_scanner);
      current_scanner = Vec::new();
    } else if i.is_empty() {
      continue;
    } else {
      let (x, y, z) = scan_fmt!(&i, "{},{},{}", i32, i32, i32).unwrap();
      current_scanner.push((x, y, z));
    }
  }
  scanners_data.push(current_scanner);
  scanners_data.into_iter().skip(1).collect_vec()
}

pub fn day19() {
  let input = read_input("./day19/input");
  let mut data = load_data(input);

  let len_scanners = data.len();
  let mut scanners_positioned = HashSet::<usize>::new();
  // let mut delta_vectors = Vec::new();
  let mut absolute_vectors = Vec::new();
  let mut data_rotated = data.clone();
  let mut data_absolute = data.clone();
  let rotations = REMAP.iter().cartesian_product(TRANSLATE.iter()).collect_vec();

  // Scanner no 0 is the one we use as (0,0,0)
  scanners_positioned.insert(0);
  for _ in 0..len_scanners {
    // delta_vectors.push((0, 0, 0));
    absolute_vectors.push((0, 0, 0));
  }

  while scanners_positioned.len() < len_scanners {
    for i in 0..len_scanners {
      for j in 0..len_scanners {
        if i == j {
          continue;
        }
        // Try to relate scanners against each other assuming one of them has not yet been figured out
        if (scanners_positioned.contains(&i) && scanners_positioned.contains(&j)) ||
          (!scanners_positioned.contains(&i) && !scanners_positioned.contains(&j)) {
          continue;
        }

        // One of them must be unpositioned
        let positioned = if scanners_positioned.contains(&i) { i } else { j };
        let unpositioned = if !scanners_positioned.contains(&i) { i } else { j };

        if data[positioned] != data_rotated[positioned] {
          data[positioned] = data_rotated[positioned].clone();
        }
        let positioned_beacons = &data[positioned];
        let unpositioned_beacons_raw = &data[unpositioned];

        let mut successfully_matched = false;

        // Here calculate all translations

        for rotation in rotations.iter() {
          let unpositioned_beacons = unpositioned_beacons_raw.iter().map(|x| translate(x, rotation)).collect_vec();
          for (x1, y1, z1) in positioned_beacons.iter() {
            for (x2, y2, z2) in unpositioned_beacons.iter() {
              if successfully_matched {
                break;
              }
              // Assume these are the same beacon and calculate delta vector
              let delta_vector = (x2-x1, y2-y1, z2-z1);

              // Now we can try matching - if there's >= 12 points in unpositioned that match positioned, you're gucci
              let potential_matches = unpositioned_beacons.iter().collect::<HashSet<&(i32, i32, i32)>>();
              let mut match_count = 0;
              for (xx, yy, zz) in positioned_beacons.iter() {
                if potential_matches.contains(&(xx + delta_vector.0, yy + delta_vector.1, zz + delta_vector.2)) {
                  match_count += 1;
                }
                if match_count >= 12 {
                  break;
                }
              }

              if match_count >= 12 {
                successfully_matched = true;
                scanners_positioned.insert(unpositioned);
                data_rotated[unpositioned] = unpositioned_beacons.clone();
                let absolute_vector = (delta_vector.0 + absolute_vectors[positioned].0, delta_vector.1 + absolute_vectors[positioned].1, delta_vector.2 + absolute_vectors[positioned].2);
                absolute_vectors[unpositioned] = absolute_vector;
                data_absolute[unpositioned] = unpositioned_beacons.iter().map(|x| (x.0 - absolute_vector.0, x.1 - absolute_vector.1, x.2 - absolute_vector.2)).collect_vec();
              }

            }
          }
        }
      }
    }
  }

  let mut absolute_positions = Vec::new();
  for i in data_absolute {
    for ii in i {
      absolute_positions.push(ii.clone());
    }
  }

  let distinct = absolute_positions.iter().collect::<HashSet<&(i32, i32, i32)>>();
  println!("{}", distinct.len());

  let mut max_manhattan = 0;
  for i in 0..len_scanners {
    for j in 0..len_scanners {
      let manhattan = (absolute_vectors[i].0 - absolute_vectors[j].0).abs() + (absolute_vectors[i].1 - absolute_vectors[j].1).abs() + (absolute_vectors[i].2 - absolute_vectors[j].2).abs();
      max_manhattan = std::cmp::max(max_manhattan, manhattan);
    }
  }
  println!("{}", max_manhattan);
}