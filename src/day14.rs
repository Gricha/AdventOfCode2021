use crate::utils::read_input;
use itertools::Itertools;
use std::{collections::HashMap, i32::MAX};

pub fn day14_1() {
    let input = read_input("./day14/input");
    let template = input[0].to_string();

    let mut rules = HashMap::<String, String>::new();
    input
        .into_iter()
        .skip(2)
        .map(|x| {
            let (a, b) = x.split(" -> ").collect_tuple().unwrap();
            (a.to_string(), b.to_string())
        })
        .for_each(|(a, b)| {
            rules.insert(a, b);
        });

    let mut current_string = template;
    for _ in 0..10 {
        let mut new_string = String::new();
        new_string.push(current_string.chars().nth(0).unwrap());
        for i in 1..current_string.len() {
            let sub = &current_string[i - 1..=i];
            let insrt = rules.get(sub).unwrap();
            // new_string.push(sub.chars().nth(0).unwrap());
            new_string.push(insrt.chars().nth(0).unwrap());
            new_string.push(sub.chars().nth(1).unwrap());
        }

        current_string = new_string;
    }

    let mut counts = HashMap::new();

    for c in current_string.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut min = MAX;
    let mut max = 0;
    for (_, v) in counts.iter() {
        if v < &min {
            min = *v;
        }
        if v > &max {
            max = *v;
        }
    }

    println!("{:?}", max - min);
}

pub fn day14() {
    let input = read_input("./day14/input");
    let template = input[0].to_string();

    let mut rules = HashMap::<String, (String, String)>::new();
    input
        .into_iter()
        .skip(2)
        .map(|x| {
            let (a, b) = x.split(" -> ").collect_tuple().unwrap();
            (a.to_string(), b.to_string())
        })
        .for_each(|(a, b)| {
            let letter_1 = a.chars().nth(0).unwrap();
            let letter_2 = a.chars().nth(1).unwrap();
            let middle = b.chars().nth(0).unwrap();
            rules.insert(
                a,
                (
                    [letter_1, middle].iter().collect(),
                    [middle, letter_2].iter().collect(),
                ),
            );
        });

    let mut current_mapping: HashMap<String, i64> = HashMap::new();
    for i in 1..template.len() {
        *current_mapping
            .entry((&template[i - 1..=i]).to_string())
            .or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut new_mapping = HashMap::new();
        for (k, v) in current_mapping.iter() {
            let (l, r) = rules.get(k).unwrap();
            *new_mapping.entry(l.to_string()).or_insert(0) += v;
            *new_mapping.entry(r.to_string()).or_insert(0) += v;
        }
        current_mapping = new_mapping;
    }

    let mut counts = HashMap::new();

    *counts.entry(template.chars().nth(0).unwrap()).or_insert(0) += 1;
    *counts.entry(template.chars().last().unwrap()).or_insert(0) += 1;

    for (k, v) in current_mapping.into_iter() {
        *counts.entry(k.chars().nth(0).unwrap()).or_insert(0) += v;
        *counts.entry(k.chars().nth(1).unwrap()).or_insert(0) += v;
    }

    let mut min: i64 = i64::MAX;
    let mut max: i64 = 0;
    for (_, v) in counts.iter() {
        let val = v / 2;
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    println!("{:?}", max - min);
}
