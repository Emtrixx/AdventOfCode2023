use crate::calendar;
use std::collections::HashMap;

pub fn run() {
    let mut numbers: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut part_one_sum = 0;
    let mut sum = 0;

    let lines = calendar::load_input(1);

    for line in lines {
        let line = line.unwrap();

        part_one_sum += part_one(&line);

        sum += part_two(&mut numbers, &line);
    }

    println!("Part One: {}", part_one_sum);
    println!("Part Two: {}", sum);
}

fn part_one(line: &String) -> u32 {
    /* First Half */
    let mut first_num = 0;
    let mut last_num = 0;

    line.chars().for_each(|c| {
        if c.is_digit(10) {
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
