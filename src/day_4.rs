use crate::utils::get_file_lines;


fn get_ranges(line: &str) -> ((i32, i32), (i32, i32)) {
    // eg. "1-3,4-6"
    let jobs: Vec<&str> = line.split(",").collect();

    let range_1: Vec<&str> = jobs[0].split("-").collect();
    let range_1: (i32, i32) = (
        range_1[0].parse().expect("Invalid number"),
        range_1[1].parse().expect("Invalid number"),
    );

    let range_2: Vec<&str> = jobs[1].split("-").collect();
    let range_2: (i32, i32) = (
        range_2[0].parse().expect("Invalid number"),
        range_2[1].parse().expect("Invalid number"),
    );

    (range_1, range_2) // The `return` keyword is not necessary here
}

fn part_1(){

    let lines = get_file_lines("data/day_4.txt");
    let mut total = 0;

    for line_result in lines {
        let line = line_result.unwrap_or_else(
            |e| panic!("Line Error {}", e));

        let (range_1, range_2) = get_ranges(&line);

        // if (first range contains second) or (second contains first)
        if (range_1.0 <= range_2.0 && range_1.1 >= range_2.1) ||
        (range_2.0 <= range_1.0 && range_2.1 >= range_1.1) {
            total += 1;
        }
    }
    println!("part_1: {}", total)
}

fn part_2(){

    let lines = get_file_lines("data/day_4.txt");
    let mut total = 0;

    for line_result in lines {
        let line = line_result.unwrap_or_else(
            |e| panic!("Line Error {}", e));

        let (range_1, range_2) = get_ranges(&line);

        // if (first_start overlaps second) or (first_end overlaps second)
        if (range_1.0 <= range_2.0 && range_2.0 <= range_1.1) ||
        (range_1.0 <= range_2.1 && range_2.0 <= range_1.1) {
            total += 1;
        }
    }
    println!("part_2: {}", total)
}
pub(crate) fn run() {
    println!("  day 4");
    part_1();
    part_2();
}