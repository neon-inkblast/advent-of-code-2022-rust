use std::fs;

pub fn part_01() {
    let s = fs::read_to_string("./src/input_02.txt").expect("couldn't read input file");

    // A / X - Rock
    // B / Y - Paper
    // C / Z - Scissors

    let mut total: i32 = 0;
    for line in s.lines() {
        let line_bytes = line.as_bytes();
        // p1 in 0..2
        let p1: i8 = (line_bytes[0] - b'A').try_into().unwrap();
        // p2 in 0..2
        let p2: i8 = (line_bytes[2] - b'X').try_into().unwrap();

        // subtract p2 from p1 and add 1,
        // then get modulo 3
        // to give:
        //   win  = 2
        //   draw = 1
        //   loss = 0
        // then multiply by 3
        //   win  = 6
        //   draw = 3
        //   loss = 0
        // NOTE: use rem_euclid() for modulo, NOT % operator, which gives remainder
        //   remainder and modulo are only equiv for dividend > 0
        let outcome = (p2 - p1 + 1).rem_euclid(3) * 3;

        // add 1 to p2 (0-2) to give
        //   rock     = 1
        //   paper    = 2
        //   scissors = 3
        let choice = p2 + 1;

        // add round score
        total += (outcome + choice) as i32;
    }
    println!("Day 2 - Part 1: {}", total);
}

pub fn part_02() {
    let s = fs::read_to_string("./src/input_02.txt").expect("couldn't read input file");

    // A / X - Rock / Lose
    // B / Y - Paper / Draw
    // C / Z - Scissors / Win

    let mut total: i32 = 0;
    for line in s.lines() {
        let line_bytes = line.as_bytes();
        // p1 in 0..2
        let p1: i8 = (line_bytes[0] - b'A').try_into().unwrap();
        // p2 in 0..2
        let p2: i8 = (line_bytes[2] - b'X').try_into().unwrap();

        // multiply p2 (0-2) by 3 to give
        //   win  = 6
        //   draw = 3
        //   loss = 0
        let choice = p2 * 3;

        // add p1 and p2 and subtract 1,
        // then get modulo 3
        // then add 1 again
        // to give:
        //   rock     = 1
        //   paper    = 2
        //   scissors = 3
        // NOTE: use rem_euclid() for modulo, NOT % operator, which gives remainder
        //   remainder and modulo are only equiv for dividend > 0
        let outcome = (p1 + p2 - 1).rem_euclid(3) + 1;

        // add round score
        total += (outcome + choice) as i32;
    }
    println!("Day 2 - Part 2: {}", total);
}
