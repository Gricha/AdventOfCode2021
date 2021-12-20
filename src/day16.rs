use itertools::Itertools;

use crate::utils::read_input;

// Will return [0,1,1,0], etc.
fn c_to_bin(c: char) -> Vec<bool> {
  let mut v: Vec<bool> = Vec::new();
  let mut car = i64::from_str_radix(&c.to_string(), 16).unwrap();
  
  while car > 0 {
    v.push((car % 2) != 0);
    car /= 2;
  }

  // Pad to 4
  if v.len() < 4 {
    for _ in 0..(4-v.len()) {
      v.push(false);
    }
  }

  v.into_iter().rev().collect_vec()
}

fn bin_to_i64(slice: &[bool]) -> i64 {
  let mut val = 0;
  slice.iter().for_each(|bit| {
    val *= 2;
    val += *bit as i64;
  });
  val
}

#[derive(Debug)]
enum PacketLengthType {
  Subpackets(i64),
  Bits(i64)
}

#[derive(Debug)]
struct PacketHeader {
  type_id: i64,
  length_type: PacketLengthType,
  subpackets: Vec<Packet>,
}

#[derive(Debug)]
enum PacketInfo {
  Literal(i64),
  Subpacket(PacketHeader)
}

#[derive(Debug)]
struct Packet {
  version: i64,
  packet_type: PacketInfo,
}

impl Packet {
  fn sum_versions(&self) -> i64 {
    if let PacketInfo::Subpacket(sub_header) = &self.packet_type {
      sub_header.subpackets.iter().fold(self.version, |mut acc, s| {
        acc += s.sum_versions();
        acc
      })
    } else {
      self.version
    }

  }

  fn evaluate(&self) -> i64 {
    match &self.packet_type {
      PacketInfo::Literal(val) => *val,
      PacketInfo::Subpacket(header) => {
        match header.type_id {
          0 => {
            header.subpackets.iter().map(|x| x.evaluate()).sum()
          },
          1 => {
            header.subpackets.iter().map(|x| x.evaluate()).product()
          },
          2 => {
            header.subpackets.iter().map(|x| x.evaluate()).min().unwrap()
          },
          3 => {
            header.subpackets.iter().map(|x| x.evaluate()).max().unwrap()
          },
          5 => {
            if header.subpackets.get(0).unwrap().evaluate() > header.subpackets.get(1).unwrap().evaluate() {
              1
            } else {
              0
            }
          },
          6 => {
            if header.subpackets.get(0).unwrap().evaluate() < header.subpackets.get(1).unwrap().evaluate() {
              1
            } else {
              0
            }
          },
          7 => {
            if header.subpackets.get(0).unwrap().evaluate() == header.subpackets.get(1).unwrap().evaluate() {
              1
            } else {
              0
            }
          },
          _ => unreachable!()
        }
      }
    }
  }
}

fn read_packet(bytes: &Vec<bool>, i: usize) -> (usize, Packet) {
  let mut index = i;
  let version = bin_to_i64( &bytes[index..index+3]);
  index += 3;
  let type_id = bin_to_i64(&bytes[index..index+3]);
  index += 3;

  if type_id == 4 {
    // Literal detected
    let mut bits = Vec::new();
    while bytes[index] == true { // 1 as first bit
      index += 1; // move to number
      bits.extend(bytes[index..index + 4].iter().cloned());
      index += 4;
    }
    // last round
    index += 1;
    bits.extend(bytes[index..index + 4].iter().cloned());
    index += 4;
    let literal = bin_to_i64(&bits);

    (index, Packet {
      version,
      packet_type: PacketInfo::Literal(literal)
    })
  } else {
    let length_type_id = bytes[index];
    index += 1;
    let length_type: PacketLengthType;
    if !length_type_id {
      let value = bin_to_i64(&bytes[index..index+15]);
      index += 15;
      length_type = PacketLengthType::Bits(value);
    } else {
      let value = bin_to_i64(&bytes[index..index+11]);
      index += 11;
      length_type = PacketLengthType::Subpackets(value);
    }

    let subpackets = match length_type {
      PacketLengthType::Bits(val) => {
        let current_index = index;
        let mut subpackets = Vec::new();
        while index < (current_index + val as usize) {
          let (new_index, packet) = read_packet(bytes, index);
          index = new_index;
          subpackets.push(packet);
        }
        subpackets
      },
      PacketLengthType::Subpackets(val) => {
        let mut subpackets = Vec::new();
        for _ in 0..val {
          let (new_index, packet) = read_packet(bytes, index);
          index = new_index;
          subpackets.push(packet);
        }
        subpackets
      }
    };

    (index, Packet {
      version, 
      packet_type: PacketInfo::Subpacket(PacketHeader {
        type_id,
        length_type,
        subpackets
      })
    })

  }
}

pub fn day16() {
  let input = read_input("./day16/input");
  let data = input.into_iter().nth(0).unwrap().chars().map(|c| c_to_bin(c))
  .fold(Vec::new(), |mut acc, v| {
    acc.extend(v);
    acc
  });

  let (_, packet) = read_packet(&data, 0);
  println!("{}", packet.evaluate());
}