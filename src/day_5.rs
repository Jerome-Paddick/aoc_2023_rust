use std::fs::File;
use std::io::{BufReader, Lines};
use crate::utils::get_file_lines;

fn build_stacks(lines: &mut Lines<BufReader<File>>) -> Vec<Vec<char>> {
    let mut line_store: Vec<String> = Vec::new();

    // Read lines until an empty line is encountered
    while let Some(line_result) = lines.next() {
        match line_result {
            Ok(line) => {
                if line.is_empty() {
                    break
                } else {
                    line_store.push(line);
                }
            }
            Err(e) => {panic!("Error reading line: {}", e);}
        }
    }

    // Ensure there is at least one line to pop
    let last_line = line_store.pop().unwrap();
    let stack_indexes = last_line.split_whitespace().collect::<Vec<_>>();
    let stack_len = stack_indexes.len();

    let mut stacks: Vec::<Vec<char>> = Vec::with_capacity(stack_len);
    for _ in 0..stack_len {
        stacks.push(Vec::new())
    }

    while let Some(curr_line) = line_store.pop(){
        //[a] [b] [c] [d]
        // 1   5   9   13
        // 4n - 3
        for n in 1..=stack_len {
            let l = curr_line.chars().nth(4*n-3)
                .expect("Expected character at calculated index");
            if l.is_alphabetic(){
                stacks[n-1].push(l)
            }
        }
    }

    stacks
}
fn part_1(){

    let mut lines = get_file_lines("data/day_5.txt");
    let mut stacks = build_stacks(&mut lines);

    while let Some(line_result) = lines.next() {
        let line = line_result.unwrap();

        let line_split: Vec<_> = line.split_whitespace().collect();
        let how_many = &line_split[1].parse::<usize>().unwrap();
        let from = &line_split[3].parse::<usize>().unwrap();
        let to = &line_split[5].parse::<usize>().unwrap();

        for _ in 0..*how_many {
            let taken_from = stacks[*from-1].pop().unwrap();
            stacks[*to-1].push(taken_from);

        // let drain_from = stacks[*from-1].len() - how_many;
        // let mut taken_from = stacks[*from-1]
        //     .drain((drain_from)..)
        //     .collect::<Vec<_>>();
        // taken_from.reverse();
        // stacks[*to - 1].append(&mut taken_from);
        }
    }

    println!("part_1: {:?}",
             stacks.iter()
            .map(|v| {
                v.last().unwrap().to_string()
            })
            .collect::<Vec<_>>()
            .join("")
    )
}


fn part_2(){

    let mut lines = get_file_lines("data/day_5.txt");
    let mut stacks = build_stacks(&mut lines);

    while let Some(line_result) = lines.next() {
        let line = line_result.unwrap();

        let line_split: Vec<_> = line.split_whitespace().collect();
        let how_many = &line_split[1].parse::<usize>().unwrap();
        let from = &line_split[3].parse::<usize>().unwrap();
        let to = &line_split[5].parse::<usize>().unwrap();

        let drain_from = stacks[*from-1].len() - how_many;
        let mut taken_from = stacks[*from-1]
            .drain((drain_from)..)
            .collect();
        stacks[*to - 1].append(&mut taken_from);
    }

    println!("part_2: {:?}",
        stacks.iter()
            .map(|v| {
                 v.last().unwrap().to_string()
                })
            .collect::<Vec<_>>()
        .join("")
    )
}
pub(crate) fn run() {
    println!("  day 5");

    // part_1();
    part_2();

}