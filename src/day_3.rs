use std::collections::HashSet;
use crate::utils::get_file_lines;


fn char_to_number(c: char) -> u16 {
    if c >= 'a' && c <= 'z' {
        (c as u16) - ('a' as u16) + 1
    } else if c >= 'A' && c <= 'Z' {
        (c as u16) - ('A' as u16) + 27
    } else {
        0
    }
}
fn part_1(){

    let lines = get_file_lines("data/day_3.txt");

    let mut total = 0;

    for line_result in lines {
        let line = line_result.unwrap_or_else(
            |e| panic!("Line Error {}", e));

        let mut unique_chars: HashSet<char> = HashSet::new();

        let half = line.len()/2;
        let first_comp = &line[0..half];
        let sec_comp = &line[half..];

        for char in first_comp.chars(){
            unique_chars.insert(char);
        }

        for char in sec_comp.chars(){
            if unique_chars.contains(&char) {
                total += char_to_number(char);
                // println!("{}", char_to_number(char));
                break
            }
        }

    }
    println!("part_1: {}", total)
}

fn part_2(){

    let mut lines = get_file_lines("data/day_3.txt");
    let mut total= 0;

    // while the next 3 lines can be returned
    while let (
        Some(first_line_result),
        Some(second_line_result),
        Some(third_line_result),
    ) = (lines.next(), lines.next(), lines.next()) {

        // Unwrap lines or panic on read error
        let first_line = first_line_result.expect("Line Error");
        let second_line = second_line_result.expect("Line Error");
        let third_line = third_line_result.expect("Line Error");

        // Create a HashSet for unique characters in the first line
        let mut first_line_unique_chars: HashSet<char> = HashSet::new();
        for char in first_line.chars() {
            first_line_unique_chars.insert(char);
        }

        // Create a HashSet for characters common between first and second lines
        let mut first_second_intersect: HashSet<char> = HashSet::new();
        for char in second_line.chars() {
            if first_line_unique_chars.contains(&char) {
                first_second_intersect.insert(char);
            }
        }

        // Check if any character in the third line is common with the intersection set
        for char in third_line.chars() {
            if first_second_intersect.contains(&char) {
                total += char_to_number(char);
                break;
            }
        }
    }
    println!("part_2: {}", total)
}
pub(crate) fn run() {
    println!("  day 3");
    part_1();
    part_2();
}