use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub mod day01;
pub mod day02;
pub mod day03;

fn load_input(day: u32) -> Lines<BufReader<File>> {
    let path = format!("src/calendar/day{:02}/input.txt", day);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    lines
}
