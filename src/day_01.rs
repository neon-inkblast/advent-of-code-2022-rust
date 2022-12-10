use std::fs;

pub fn part_01() {
    // include_str reads the file and includes it at compile time
    // include_str!("./input_01.txt"));
    // read_to_string reads file at runtime (ROOT is base of path)
    let s = fs::read_to_string("./src/input_01.txt").expect("couldn't read input file");
    let mut elves: Vec<u32> = Vec::new();
    let mut curr_sum: u32 = 0;
    // sum up pack calories
    for line in s.lines() {
        if line.len() == 0 {
            elves.push(curr_sum);
            curr_sum = 0;
        } else {
            curr_sum += line.parse::<u32>().unwrap();
        }
    }
    // sort the vector
    elves.sort();
    // take last
    let res: &u32 = elves.last().expect("list empty");
    println!("Day 1 - Part 1: {}", res)
}

pub fn part_02() {
    let s = fs::read_to_string("./src/input_01.txt").expect("couldn't read input file");
    let mut elves: Vec<u32> = Vec::new();
    let mut curr_sum: u32 = 0;
    for line in s.lines() {
        if line.len() == 0 {
            elves.push(curr_sum);
            curr_sum = 0;
        } else {
            curr_sum += line.parse::<u32>().unwrap();
        }
    }

    // sort the vector
    elves.sort();
    // reverse it
    elves.reverse();
    // drop the rest
    elves.truncate(3);
    // convert to an iter and use sum()
    let res: u32 = elves.iter().sum();
    // print result
    println!("Day 1 - Part 2: {}", res)
}

// for each line
// split string in half (length/2)
// for each letter in half 1, see if it's in half 2
// only 1 will match, save that
