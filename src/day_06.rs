use std::{collections::HashSet, fs, str::from_utf8};

use termion::{color, style};

pub fn part_01() -> usize {
    let s = fs::read_to_string("./src/input_06.txt").expect("couldn't read input file");
    let input = s.lines().take(1).next().expect("oh noes").as_bytes();
    let back_check = 4;
    let mut set: HashSet<u8>;
    for i in back_check..input.len() {
        set = (i - back_check..i).map(|n| input[n]).collect();
        if set.len() == back_check {
            print!("Day 6 - Part 1: {} ", i,);
            print!(
                "{}{}",
                color::Fg(color::Green),
                from_utf8(&input[i - back_check..i]).unwrap()
            );
            println!(
                "{}{}{}",
                color::Fg(color::Blue),
                from_utf8(&input[i..i + 50]).unwrap(),
                style::Reset
            );
            return i;
        }
        set.clear();
    }
    return 0;
}

pub fn part_02() -> usize {
    let s = fs::read_to_string("./src/input_06.txt").expect("couldn't read input file");
    let input = s.lines().take(1).next().expect("oh noes").as_bytes();
    let back_check = 14;
    let mut set: HashSet<u8>;
    for i in back_check..input.len() {
        set = (i - back_check..i).map(|n| input[n]).collect();
        if set.len() == back_check {
            print!("Day 6 - Part 2: {} ", i,);
            print!(
                "{}{}",
                color::Fg(color::Green),
                from_utf8(&input[i - back_check..i]).unwrap()
            );
            println!(
                "{}{}{}",
                color::Fg(color::Blue),
                from_utf8(&input[i..i + 50]).unwrap(),
                style::Reset
            );
            return i;
        }
        set.clear();
    }
    return 0;
}
