use array2d::{Array2D, Error};
use core::time;
use std::{fs, thread};

// use termion::{color, style};

pub fn part_01() -> usize {
    let s = fs::read_to_string("./src/input_10.txt").expect("couldn't read input file");

    return 0;
}

pub fn part_02() -> usize {
    let s = fs::read_to_string("./src/input_10.txt").expect("couldn't read input file");
    let timeout = time::Duration::from_millis(25);
    let mut sprite: [i32; 3] = [0, 1, 2];
    let mut cursor: [usize; 2] = [0, 0];
    let mut display: Array2D<char> = Array2D::filled_with(' ', 6, 40);

    let words = s.lines().flat_map(|x| x.split(" ")).collect::<Vec<&str>>();
    for inst in words {
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

        match inst.parse::<i32>() {
            Ok(addx) => {
                sprite = sprite.map(|x| x + addx);
            }
            Err(_) => continue,
        };
    }

    let disp_rows = display.as_rows();
    for i in 0..disp_rows.len() {
        let st: String = disp_rows[i].iter().collect();
        for j in 0..39 {
            print!("{}", st.chars().nth(j).expect("died"));
            thread::sleep(timeout);
        }
        println!("{}", st.chars().nth(39).unwrap());
        thread::sleep(timeout);
    }
    return 0;
}
