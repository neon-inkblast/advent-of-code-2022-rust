use array2d::Array2D;
use core::time;
use std::{
    fs,
    io::{self, Write},
    thread,
};

use termion::{
    color::{self},
    style,
};

pub fn part_01() -> usize {
    let s = fs::read_to_string("./src/input_10.txt").expect("couldn't read input file");

    let mut reg_x: i32 = 1;
    let mut cycle: i32 = 1;
    let mut signal: i32 = 0;
    let offset: i32 = 20;

    let words = s.lines().flat_map(|x| x.split(" ")).collect::<Vec<&str>>();
    for instr in words {
        cycle += 1;
        if (cycle + offset) % 40 == 0 {
            signal += cycle * reg_x;
        }

        match instr.parse::<i32>() {
            Ok(addx) => reg_x += addx,
            Err(_) => continue,
        };
    }
    println!("Day 10 - Part 1: {}", signal);

    return 0;
}

pub fn part_02() -> usize {
    let s = fs::read_to_string("./src/input_10.txt").expect("couldn't read input file");
    let timeout = time::Duration::from_millis(25);
    let mut sprite: [i32; 3] = [0, 1, 2];
    let mut cursor: [usize; 2] = [0, 0];
    let mut display: Array2D<char> = Array2D::filled_with(' ', 6, 40);

    let words = s.lines().flat_map(|x| x.split(" ")).collect::<Vec<&str>>();
    for instr in words {
        for h_pos in sprite {
            if cursor[1] as i32 == h_pos {
                display
                    .set(cursor[0], cursor[1], '█')
                    .expect("Out of bounds on display!");
            }
        }
        cursor[1] = cursor[1] + 1;
        if cursor[1] >= 40 {
            cursor[0] = cursor[0] + 1;
            cursor[1] = 0;
        }

        match instr.parse::<i32>() {
            Ok(addx) => {
                sprite = sprite.map(|x| x + addx);
            }
            Err(_) => continue,
        };
    }

    // let colours: [&str; 8] = [
    //     color::Blue.fg_str(),
    //     color::Green.fg_str(),
    //     color::LightYellow.fg_str(),
    //     color::Red.fg_str(),
    //     color::Magenta.fg_str(),
    //     color::Cyan.fg_str(),
    //     color::LightGreen.fg_str(),
    //     color::LightRed.fg_str(),
    // ];
    // let colours: [&str; 8] = [
    //     color::Green.fg_str(),
    //     color::LightGreen.fg_str(),
    //     color::Green.fg_str(),
    //     color::LightGreen.fg_str(),
    //     color::Green.fg_str(),
    //     color::LightGreen.fg_str(),
    //     color::Green.fg_str(),
    //     color::LightGreen.fg_str(),
    // ];

    let disp_rows = display.as_rows();
    println!("Day 10 - Part 2: ");
    println!(
        "{}  ╔═[ Elf OS 2022 ]═══════════════════════════╗",
        color::LightGreen.fg_str(),
        //style::Reset
    );
    println!("  ║                                           ║");

    for i in 0..disp_rows.len() {
        let row_text: String = disp_rows[i].iter().collect();
        print!("  ║  ");
        for j in 0..39 {
            print!(
                "{}{}",
                color::Cyan.fg_str(), // colours[(i).min(7)],
                row_text.chars().nth(j).expect("died"),
            );
            io::stdout().flush().expect("Died whilst flushing");
            thread::sleep(timeout);
        }
        println!(
            "{}{} ║",
            row_text.chars().nth(39).unwrap(),
            color::LightGreen.fg_str()
        );
        thread::sleep(timeout);
    }
    println!("  ║                                           ║");
    println!(
        "  ╚═══════════════════════════════════════════╝{}",
        style::Reset
    );
    return 0;
}
