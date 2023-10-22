use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Read;
use crate::utils::get_file_buffer;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn part_1(){
    // 1361

    // have only ascii chars -> utf-8 single byte chars -> can iterate over using .bytes()
    // chars will convert to unicode scalar values -> 4 byte chars regardless of underlying length
    let mut bytes = get_file_buffer("data/day_7.txt").bytes();
    let mut window = Vec::with_capacity(4);

    // by_ref -> does not consume iterator so it can be used later
    window.extend(bytes.by_ref().take(3).filter_map(Result::ok));

    let mut hash_set:HashSet<u8> = HashSet::with_capacity(4);
    let mut end = 4;

    while let Some(byte_result) = bytes.next(){

        let byte = byte_result.expect("Could not read byte from file");
        window.push(byte);
        hash_set.extend(&window);

        if hash_set.len() == 4 {
            break;
        } else {
            window.remove(0);
            hash_set.clear();
            end += 1;
        }
    }

    println!("part_1: {}", end);

}


fn part_2() {
    // 3263

    // number of consecutive unique characters to find
    let n = 14;

    // have only ascii chars == utf-8 single byte chars -> can iterate over using .bytes()
    // chars will convert to unicode scalar values -> 4 byte chars regardless of underlying length
    let mut bytes = get_file_buffer("data/day_6.txt").bytes();

    // non contiguous memory allocation -> can efficiently pop from front or back
    let mut window: VecDeque<u8> = VecDeque::with_capacity(n);
    // counts of letters in last 14 read bytes
    let mut history: HashMap<u8, i32> = HashMap::with_capacity(n);
    // current length of history (number of unique letters)
    let mut curr = 0;

    window.extend(bytes.by_ref().take(n).filter_map(Result::ok));

    // a reference to an iterator is an iterator of references to the values in the iter
    // &byte is pattern matching a reference, and so returns the a de-referenced value
    // -> in this case a copy as u8 has the copy trait
    // -> u8 is likely smaller than a reference to a u8
    for &byte in &window {
        match history.entry(byte) {
            Vacant(entry) => {
                entry.insert(1);
                curr += 1;
            }
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
        }
    }

    let mut end = 14;

    while let Some(byte_result) = bytes.next() {
        // println!("{}", curr);
        // println!("{:?}", history.clone().into_iter()
        //     .map(|(key, value)| (key as char, value)).collect::<Vec<_>>());
        // println!("{:?}", window.iter().map(|&c| c as char).collect::<Vec<_>>());

        if curr == 14 {
            break
        }

        end += 1;
        let byte = byte_result.expect("Could not read byte from file");

        // add to end of window
        match history.entry(byte) {
            Vacant(entry) => {
                entry.insert(1);
                curr += 1;
            }
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
        }

        // remove start of window
        if let Some(start) = window.pop_front() {
            // get and get_mut takes a reference to the key
            // so it can handle types that do not implement the copy trait
            let count = history.get_mut(&start)
                .expect("Start byte not found in history");
            if *count == 1 {
                history.remove(&start);
                curr -= 1;
            } else {
                *count -= 1;
            }
        }
        window.push_back(byte);
    }

    println!("part_2: {}", end);
}

pub(crate) fn run() {
    println!("  day 7");

    // let start = Instant::now();
    // println!("Time elapsed in my_function() is: {:?}", start.elapsed());

    // t=2.9ms
    // part_1();

    // using part_1 method t=17.5ms
    // using part_2 method t=3.8ms
    // part_2();

}