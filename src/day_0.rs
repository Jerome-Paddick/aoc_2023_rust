use crate::utils::get_file;

fn part_1(){

    let lines = get_file("_data/day_5.txt");
    for line_result in lines {
        let line = line_result.unwrap_or_else(
            |e| panic!("Line Error {}", e));

        println!("{}", line)

    }
    println!("part_1: {}", "")
}

fn part_2(){

    let lines = get_file("data/day_4.txt");

    for line_result in lines {
        let line = line_result.unwrap_or_else(
            |e| panic!("Line Error {}", e));

    }
    println!("part_2: {}", "")
}
pub(crate) fn run() {
    println!("  day 5");

    // let start = Instant::now();
    // println!("Time elapsed in my_function() is: {:?}", start.elapsed());

    part_1();
    // part_2();
}