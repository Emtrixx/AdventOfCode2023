use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn run() {
    let mut numbers: HashMap<&str, u32> = HashMap::new();
    numbers.insert("one", 1);
    numbers.insert("two", 2);
    numbers.insert("three", 3);
    numbers.insert("four", 4);
    numbers.insert("five", 5);
    numbers.insert("six", 6);
    numbers.insert("seven", 7);
    numbers.insert("eight", 8);
    numbers.insert("nine", 9);
    numbers.insert("1", 1);
    numbers.insert("2", 2);
    numbers.insert("3", 3);
    numbers.insert("4", 4);
    numbers.insert("5", 5);
    numbers.insert("6", 6);
    numbers.insert("7", 7);
    numbers.insert("8", 8);
    numbers.insert("9", 9);

    let mut part_one_sum = 0;
    let mut sum = 0;

    let lines = load_input();

    for line in lines {
        let line = line.unwrap();

        part_one_sum += part_one(&line);

        sum += part_two(&mut numbers, &line);
    }

    println!("Part One: {}", part_one_sum);
    println!("Part Two: {}", sum);
}

fn load_input() -> Lines<BufReader<File>> {
    let file = File::open("../day01/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines();
    lines
}

fn part_two(numbers: &mut HashMap<&str, u32>, line: &String) -> u32 {
    /* Second Half */
    let mut first_num = 0;
    let mut last_num = 0;
    let mut smallest_index = 1000;
    let mut biggest_index = 0;

    for num in numbers.iter() {
        if let Some(index) = line.find(num.0) {
            if index < smallest_index {
                smallest_index = index;
                first_num = *num.1;
            }
        }
        if let Some(index) = line.rfind(num.0) {
            if index >= biggest_index {
                biggest_index = index;
                last_num = *num.1;
            }
        }
    }

    return (first_num * 10) + last_num;
}

fn part_one(line: &String) -> u32 {
    /* First Half */
    let mut first_num = 0;
    let mut last_num = 0;

    line.chars().for_each(|c| {
        if (c.is_digit(10)) {
            let num = c.to_digit(10).unwrap();
            if first_num == 0 {
                first_num = num;
                last_num = num;
            } else {
                last_num = num;
            }
        }
    });
    return (first_num * 10) + last_num;
}
