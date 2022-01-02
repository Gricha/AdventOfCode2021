use crate::utils::read_input;

use std::collections::HashMap;

pub fn day21_1() {
    let input = read_input("./day21/input");
    let mut start_1 = scan_fmt!(&input[0], "Player 1 starting position: {}", usize).unwrap() - 1;
    let mut start_2 = scan_fmt!(&input[1], "Player 2 starting position: {}", usize).unwrap() - 1;

    let mut score_1 = 0;
    let mut score_2 = 0;

    let mut dice_pos = 1;
    let mut player_1 = true;
    let mut number_of_rolls = 0;
    loop {
        let mut m = 0;
        for _ in 0..3 {
            m += dice_pos;
            number_of_rolls += 1;
            dice_pos += 1;
            if dice_pos > 100 {
                dice_pos = 1;
            }
        }

        if player_1 {
            start_1 += m;
            start_1 = start_1 % 10;
            score_1 += start_1 + 1;
        } else {
            start_2 += m;
            start_2 = start_2 % 10;
            score_2 += start_2 + 1;
        }
        player_1 = !player_1;

        if score_1 >= 1000 || score_2 >= 1000 {
            break;
        }
    }
    println!("{}", std::cmp::min(score_2, score_1) * number_of_rolls);
}

type PlayerState = (usize, usize, usize, usize); // (Position, Score)

type Universes = HashMap<PlayerState, (usize, usize)>;

fn get_wins_from_position(
    universes: &mut Universes,
    rolls: &[usize],
    pos_1: usize,
    score_1: usize,
    pos_2: usize,
    score_2: usize,
    is_player_1_moving: bool,
) -> (usize, usize) {
    if universes.contains_key(&(pos_1, score_1, pos_2, score_2)) {
        return *universes.get(&(pos_1, score_1, pos_2, score_2)).unwrap();
    }
    if score_1 >= 21 {
        return (1, 0);
    }
    if score_2 >= 21 {
        return (0, 1);
    }

    let mut final_scores = (0, 0);
    for i in 3..10 {
        let scores: (usize, usize);
        let new_position = ((pos_1 + i - 1) % 10) + 1;
        let new_score = score_1 + new_position;
        scores = get_wins_from_position(
            universes,
            rolls,
            pos_2,
            score_2,
            new_position,
            new_score,
            !is_player_1_moving,
        );
        final_scores.0 += rolls[i] * scores.1;
        final_scores.1 += rolls[i] * scores.0;
    }

    universes.insert((pos_1, score_1, pos_2, score_2), final_scores);
    final_scores
}

pub fn day21() {
    let input = read_input("./day21/input");
    let start_1 = scan_fmt!(&input[0], "Player 1 starting position: {}", usize).unwrap();
    let start_2 = scan_fmt!(&input[1], "Player 2 starting position: {}", usize).unwrap();

    let mut universes = Universes::new();

    let mut possible_roles = vec![0; 10];
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                possible_roles[i + j + k] += 1;
            }
        }
    }

    let (wins_1, wins_2) = get_wins_from_position(
        &mut universes,
        &possible_roles,
        start_1,
        0,
        start_2,
        0,
        true,
    );

    println!("{} {}", wins_1, wins_2);
}
