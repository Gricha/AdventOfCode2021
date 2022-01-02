use itertools::Itertools;
use std::cmp::min;

use crate::utils::read_input;

pub fn day15_1() {
    let input = read_input("./day15/input");

    let mut risk_map: Vec<Vec<i64>> = vec![vec![i32::MAX as i64; 102]; 102];

    let res = input
        .into_iter()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| c as i32 - '0' as i32)
                .collect_vec()
        })
        .collect_vec();

    for i in 0..100 {
        for j in 0..100 {
            risk_map[i + 1][j + 1] = res[i][j] as i64;
        }
    }

    let mut path_cost: Vec<Vec<i64>> = vec![vec![i32::MAX as i64; 102]; 102];
    path_cost[1][1] = 0;

    // Recursively propagate risk
    fn update_risk(path_cost: &mut Vec<Vec<i64>>, risk_map: &Vec<Vec<i64>>, i: usize, j: usize) {
        if i == 0 || j == 0 || i == 101 || j == 101 {
            return;
        }

        let smallest_cost = min(
            min(path_cost[i - 1][j], path_cost[i + 1][j]),
            min(path_cost[i][j - 1], path_cost[i][j + 1]),
        ) + risk_map[i][j];

        if path_cost[i][j] <= smallest_cost {
            return;
        }

        path_cost[i][j] = smallest_cost;

        update_risk(path_cost, risk_map, i - 1, j);
        update_risk(path_cost, risk_map, i, j - 1);
    }

    for i in 0..100 {
        for j in 0..100 {
            update_risk(&mut path_cost, &risk_map, i + 1, j + 1);
        }
    }

    println!("{:?}", path_cost[100][100]);
}

pub fn day15() {
    let input = read_input("./day15/input");

    let mut risk_map: Vec<Vec<i64>> = vec![vec![i32::MAX as i64; 502]; 502];

    let res = input
        .into_iter()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| c as i32 - '0' as i32)
                .collect_vec()
        })
        .collect_vec();

    for i in 0..500 {
        for j in 0..500 {
            let val: i64 = res[i % 100][j % 100] as i64;
            let risk_level_rise = (i / 100) + (j / 100);
            let mut risk_level: i64 = (val + risk_level_rise as i64) % 9;
            if risk_level == 0 {
                risk_level = 9;
            }
            risk_map[i + 1][j + 1] = risk_level;
        }
    }

    let mut path_cost: Vec<Vec<i64>> = vec![vec![i32::MAX as i64; 502]; 502];
    path_cost[1][1] = 0;

    // Recursively propagate risk
    fn update_risk(path_cost: &mut Vec<Vec<i64>>, risk_map: &Vec<Vec<i64>>, i: usize, j: usize) {
        if i == 0 || j == 0 || i == 501 || j == 501 {
            return;
        }

        let smallest_cost = min(
            min(path_cost[i - 1][j], path_cost[i + 1][j]),
            min(path_cost[i][j - 1], path_cost[i][j + 1]),
        ) + risk_map[i][j];

        if path_cost[i][j] <= smallest_cost {
            return;
        }

        path_cost[i][j] = smallest_cost;

        update_risk(path_cost, risk_map, i - 1, j);
        update_risk(path_cost, risk_map, i, j - 1);
    }

    for i in 0..500 {
        for j in 0..500 {
            update_risk(&mut path_cost, &risk_map, i + 1, j + 1);
        }
    }

    println!("{:?}", path_cost[500][500]);
}
