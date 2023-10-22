use std::collections::HashMap;
use crate::utils::get_file_lines;

fn part_1(){

    // score for player choice
    let mut player_choice_score_map = HashMap::with_capacity(3);
    player_choice_score_map.insert('X', 1);
    player_choice_score_map.insert('Y', 2);
    player_choice_score_map.insert('Z', 3);

    // maps all possible outcomes
    let mut opp_chose_rock = HashMap::with_capacity(3);
    opp_chose_rock.insert('X', 3);  // rock -> draw
    opp_chose_rock.insert('Y', 6);  // paper -> win
    opp_chose_rock.insert('Z', 0);  // sci -> lose

    let mut opp_chose_paper = HashMap::with_capacity(3);
    opp_chose_paper.insert('X', 0);  // rock -> lose
    opp_chose_paper.insert('Y', 3);  // paper -> draw
    opp_chose_paper.insert('Z', 6);  // sci -> win

    let mut opp_chose_sci = HashMap::with_capacity(3);
    opp_chose_sci.insert('X', 6);  // rock -> win
    opp_chose_sci.insert('Y', 0);  // paper -> lose
    opp_chose_sci.insert('Z', 3);  // sci -> draw

    // score for round part_1 :  opp_move player_move
    let mut game_score_map_part_1 = HashMap::with_capacity(3);
    game_score_map_part_1.insert('A', &opp_chose_rock);
    game_score_map_part_1.insert('B', &opp_chose_paper);
    game_score_map_part_1.insert('C', &opp_chose_sci);

    let mut total_player_choice:i32 = 0;
    let mut total_score:i32 = 0;

    let lines = get_file_lines("data/day_2.txt");

    for line_result in lines {
        let line = line_result.unwrap_or_else(
            |e| panic!("Line Error {}", e));
        let chars: Vec<char> = line.chars().collect();
        let opp_move: char = chars[0];
        let player_move: char = chars[2];
        total_player_choice += player_choice_score_map[&player_move];
        total_score += game_score_map_part_1[&opp_move][&player_move];
    }
    println!("part_1 {}", total_player_choice + total_score);
}

fn part_2(){

    // player choice score part_2
    let mut player_choice_score = HashMap::with_capacity(3);
    player_choice_score.insert('X', 0); // lose
    player_choice_score.insert('Y', 3); // draw
    player_choice_score.insert('Z', 6); // win

    // maps all possible outcomes:  opp -> player
    let mut player_lose_map = HashMap::with_capacity(3);
    player_lose_map.insert('A', 3);  // rock -> sci
    player_lose_map.insert('B', 1);  // paper -> rock
    player_lose_map.insert('C', 2);  // sci -> paper

    let mut player_draw_map = HashMap::with_capacity(3);
    player_draw_map.insert('A', 1);  // rock -> rock
    player_draw_map.insert('B', 2);
    player_draw_map.insert('C', 3);

    let mut player_win_map = HashMap::with_capacity(3);
    player_win_map.insert('A', 2);  // rock -> paper
    player_win_map.insert('B', 3);  // paper -> sci
    player_win_map.insert('C', 1);  // sci -> rock

    //
    let mut game_move_from_outcome = HashMap::with_capacity(3);
    game_move_from_outcome.insert('X', player_lose_map);
    game_move_from_outcome.insert('Y', player_draw_map);
    game_move_from_outcome.insert('Z', player_win_map);


    let mut total_player_choice:i32 = 0;
    let mut total_score:i32 = 0;

    let lines = get_file_lines("data/day_2.txt");
    for line_result in lines {
        let line = line_result.unwrap_or_else(
            |e| panic!("Line Error {}", e));
        let chars: Vec<char> = line.chars().collect();
        let opp_move: char = chars[0];
        let outcome: char = chars[2];
        total_score += player_choice_score[&outcome];
        total_player_choice += game_move_from_outcome[&outcome][&opp_move];
    }

    println!("part_2 {}", total_player_choice + total_score);

}
pub(crate) fn run() {

    println!("  day 2");
    part_1();
    part_2();
}