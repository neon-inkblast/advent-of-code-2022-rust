use std::thread;

pub mod day_01;
pub mod day_06;
fn main() {
    day_01::part_01();
    day_01::part_02();
    let handle = thread::spawn(|| {
        day_06::part_01();
    });
    day_06::part_02();

    handle.join().unwrap();
}
