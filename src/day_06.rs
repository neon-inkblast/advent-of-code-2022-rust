use core::time;
use std::{collections::HashSet, fs, str::from_utf8, thread};

use termion::{clear, color, cursor::Goto};

pub fn part_01() -> usize {
    let s = fs::read_to_string("./src/input_06.txt").expect("couldn't read input file");
    let input = s.lines().take(1).next().expect("oh noes").as_bytes();
    let back_check = 4;
    let mut set: HashSet<u8>;
    let timeout = time::Duration::from_millis(10);
    print!("{}", clear::All);

    for i in back_check..input.len() {
        set = (i - back_check..i).map(|n| input[n]).collect();
        if set.len() == back_check {
            print!(
                "{}{}{}",
                Goto(1, 1),
                color::Fg(color::Green),
                from_utf8(&input[i - back_check..i]).unwrap()
            );
            println!(
                "{}{}",
                color::Fg(color::Blue),
                from_utf8(&input[i..i + 50]).unwrap()
            );
            println!("{}{}", color::Fg(color::Green), i);
            return i;
        }
        print!(
            "{}{}{}",
            Goto(1, 1),
            color::Fg(color::Red),
            from_utf8(&input[i - back_check..i]).unwrap()
        );
        println!(
            "{}{}{}",
            Goto((back_check + 1).try_into().unwrap(), 1),
            color::Fg(color::Blue),
            from_utf8(&input[i..i + 50]).unwrap()
        );
        set.clear();

        thread::sleep(timeout);
    }
    return 0;
}

pub fn part_02() -> usize {
    let s = fs::read_to_string("./src/input_06.txt").expect("couldn't read input file");
    let input = s.lines().take(1).next().expect("oh noes").as_bytes();
    let back_check = 14;
    let mut set: HashSet<u8>;
    let timeout = time::Duration::from_millis(10);
    for i in back_check..input.len() {
        set = (i - back_check..i).map(|n| input[n]).collect();
        if set.len() == back_check {
            print!(
                "{}{}{}",
                Goto(1, 4),
                color::Fg(color::Green),
                from_utf8(&input[i - back_check..i]).unwrap()
            );
            println!(
                "{}{}",
                color::Fg(color::Blue),
                from_utf8(&input[i..i + 50]).unwrap()
            );
            println!("{}{}", color::Fg(color::Green), i);
            return i;
        }
        print!(
            "{}{}{}",
            Goto(1, 4),
            color::Fg(color::Red),
            from_utf8(&input[i - back_check..i]).unwrap()
        );
        println!(
            "{}{}{}",
            Goto((back_check + 1).try_into().unwrap(), 4),
            color::Fg(color::Blue),
            from_utf8(&input[i..i + 40]).unwrap()
        );
        set.clear();
        thread::sleep(timeout);
    }
    return 0;
}
