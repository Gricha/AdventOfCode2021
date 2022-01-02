use crate::utils::read_input;

use std::{collections::HashMap, collections::HashSet};

#[derive(Debug)]
struct Signal {
    patterns: Vec<String>, // 10
    values: Vec<String>,   // 4
}

fn assemble_sorted_word(mut characters: Vec<char>) -> String {
    characters.sort();
    return characters.into_iter().collect();
}

impl Signal {
    fn get_output(&self) -> i32 {
        // Can get that by adding letters in 4+7 and checking what's left over
        let mut num_1: &str = "z";
        let mut num_7: &str = "z";
        let mut num_4: &str = "z";
        let mut num_6: Vec<&str> = Vec::new();
        for p in self.patterns.iter() {
            if p.len() == 4 {
                num_4 = p;
            } else if p.len() == 3 {
                num_7 = p;
            } else if p.len() == 2 {
                num_1 = p;
            } else if p.len() == 6 {
                num_6.push(p);
            }
        }

        let mut top = 'z';
        let chars_1: HashSet<char> = num_1.chars().collect();
        for c in num_7.chars() {
            if !chars_1.contains(&c) {
                top = c;
            }
        }

        let mut chars: HashSet<char> = num_4.chars().collect();
        let chars_7: HashSet<char> = num_7.chars().collect();
        chars.extend(&chars_7);

        let remainder: Vec<char> = "abcdefg"
            .chars()
            .collect::<HashSet<char>>()
            .difference(&chars)
            .cloned()
            .collect();

        let mut bottom_left = 'z';
        let mut bottom = 'z';

        let mut pattern_representing_9 = "z";
        let mut pattern_representing_6 = "z";
        let mut pattern_representing_0 = "z";
        let mut top_right = 'z';
        let mut bottom_right = 'z';

        for p in num_6.iter() {
            let p_chars = p.chars().collect::<HashSet<char>>();

            for i in 0..2 {
                if !p_chars.contains(&remainder[i]) {
                    // We know that this digit we just found is 9, and the missing character is lower bottom
                    // Return bottom left first, and bottom second
                    bottom_left = remainder[i];
                    bottom = remainder[1 - i];
                    pattern_representing_9 = p;
                }
            }
        }
        for p in num_6.iter() {
            let chars_1_as_vec: Vec<char> = chars_1.iter().cloned().collect();
            if p != &pattern_representing_9 {
                let p_chars = p.chars().collect::<HashSet<char>>();
                for i in 0..2 {
                    if !p_chars.contains(&chars_1_as_vec[i]) {
                        // We know that p is 6
                        pattern_representing_6 = p;
                        // We also know that the char that wasn't found is top-right
                        top_right = chars_1_as_vec[i];
                        bottom_right = chars_1_as_vec[1 - i];
                    }
                }
            }
        }
        for p in num_6 {
            if p != pattern_representing_9 && p != pattern_representing_6 {
                pattern_representing_0 = p;
            }
        }

        let chars_0: HashSet<char> = pattern_representing_0.chars().collect();
        let middle_char_vec: Vec<char> = "abcdefg"
            .chars()
            .into_iter()
            .collect::<HashSet<char>>()
            .difference(&chars_0)
            .cloned()
            .collect();
        assert!(middle_char_vec.len() == 1);
        let middle = middle_char_vec[0];

        let all_found = vec![top, top_right, middle, bottom_left, bottom_right, bottom];
        let all_found_set = all_found.into_iter().collect::<HashSet<char>>();
        let top_left_char_vec: Vec<char> = "abcdefg"
            .chars()
            .into_iter()
            .collect::<HashSet<char>>()
            .difference(&all_found_set)
            .cloned()
            .collect();
        assert!(top_left_char_vec.len() == 1);
        let top_left = top_left_char_vec[0];
        let match_0 = assemble_sorted_word(vec![
            top,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            bottom,
        ]);
        let match_1 = assemble_sorted_word(vec![top_right, bottom_right]);
        let match_2 = assemble_sorted_word(vec![top, top_right, middle, bottom_left, bottom]);
        let match_3 = assemble_sorted_word(vec![top, top_right, middle, bottom_right, bottom]);
        let match_4 = assemble_sorted_word(vec![top_left, top_right, middle, bottom_right]);
        let match_5 = assemble_sorted_word(vec![top, top_left, middle, bottom_right, bottom]);
        let match_6 = assemble_sorted_word(vec![
            top,
            top_left,
            middle,
            bottom_left,
            bottom_right,
            bottom,
        ]);
        let match_7 = assemble_sorted_word(vec![top, top_right, bottom_right]);
        let match_8 = assemble_sorted_word(vec![
            top,
            top_left,
            top_right,
            middle,
            bottom_left,
            bottom_right,
            bottom,
        ]);
        let match_9 =
            assemble_sorted_word(vec![top, top_left, top_right, middle, bottom_right, bottom]);

        let mut mapping = HashMap::<String, i32>::new();
        mapping.insert(match_0, 0);
        mapping.insert(match_1, 1);
        mapping.insert(match_2, 2);
        mapping.insert(match_3, 3);
        mapping.insert(match_4, 4);
        mapping.insert(match_5, 5);
        mapping.insert(match_6, 6);
        mapping.insert(match_7, 7);
        mapping.insert(match_8, 8);
        mapping.insert(match_9, 9);

        let values: Vec<i32> = self
            .values
            .clone()
            .into_iter()
            .map(|x| -> String {
                let mut chars = x.chars().collect::<Vec<char>>();
                chars.sort();
                chars.into_iter().collect()
            })
            .map(|x| mapping[&x])
            .collect();

        values[0] * 1000 + values[1] * 100 + values[2] * 10 + values[3]
    }
}

fn split_string(val: &str) -> Vec<&str> {
    val.split(" ").filter(|x| !x.is_empty()).collect()
}

pub fn day8() {
    let input = read_input("./day8/input");

    let mut signals = Vec::new();
    for i in input {
        let components: Vec<&str> = i.split("|").collect();
        let patterns_str = components[0];
        let values_str = components[1];

        let patterns: Vec<String> = split_string(patterns_str)
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let values: Vec<String> = split_string(values_str)
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        signals.push(Signal { patterns, values });
    }

    let mut count = 0;
    for signal in signals.iter() {
        for val in &signal.values {
            if [2, 3, 4, 7].contains(&val.len()) {
                count += 1;
            }
        }
    }

    println!("{}", count);

    let result: i32 = signals.iter().map(|x| x.get_output()).into_iter().sum();
    println!("{}", result);
}
