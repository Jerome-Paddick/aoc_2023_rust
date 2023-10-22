use crate::utils::{get_file_lines};


fn part_1(){
    let mut curr: i32 = 0;
    let mut highest: i32 = 0;

    let lines = get_file_lines("data/day_1.txt");
    for line_result in lines {
        let line = line_result.unwrap_or_else(
            |e| panic!("Line Error {}", e));
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            if curr > highest { highest = curr; }
            curr = 0;
        } else {
            curr += trimmed_line.parse::<i32>()
                .unwrap_or_else(|_| panic!("Line does not contain an integer"));
        }
    }
    println!("part_1 {}", highest)
}

fn part_2(){

    let mut curr:i32 = 0;
    let mut output_arr:[i32; 3] = [0, 0, 0];

    let lines = get_file_lines("data/day_1.txt");
    for line_result in lines {
        let line = line_result.unwrap_or_else(
            |e| panic!("Line Error {}", e));
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            if curr > output_arr[0]{
                output_arr[0] = curr;
            }
            curr = 0;
            output_arr.sort();
        } else {
            curr += trimmed_line.parse::<i32>()
                .unwrap_or_else(|_| panic!("Line does not contain an integer"));
        }
    }
    // println!("The highest 3 calorie counts are {}",
    //          output_arr.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));
    println!("part_2 {}", output_arr.iter().sum::<i32>())
}
pub(crate) fn run() {
    println!("  day 1");
    part_1();
    part_2();
}

