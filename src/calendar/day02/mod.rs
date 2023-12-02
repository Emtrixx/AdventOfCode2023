use crate::calendar;
use regex::Regex;

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

pub fn run() {
    // Remove Game: 1:... from each line
    let re = Regex::new(r"Game \d+: ").unwrap();
    let lines: Vec<String> = calendar::load_input(2)
        .map(|line| {
            let line = line.unwrap();
            re.replace(&line, "").to_string()
        })
        .collect::<Vec<String>>();

    // Sum ids of games that are possible
    let id_sum = part_one(&lines);

    // Sum the power of cubes of the smallest possible set of cubes for each game
    let power_cubes = part_two(lines);

    println!("Part One: {}", id_sum);
    println!("Part Two: {}", power_cubes);
}

fn part_two(lines: Vec<String>) -> u32 {
    let mut power_cubes: u32 = 0;

    for line in lines.into_iter() {
        let mut colors: [u32; 3] = [0; 3];

        let draws = line.split(";").collect::<Vec<&str>>();
        for draw in draws {
            let draw = draw.trim();
            let numbers = draw.split(",").collect::<Vec<&str>>();
            for number in numbers {
                let number = number.trim();
                let pair = number.split(" ").collect::<Vec<&str>>();
                let color = pair[1];
                let digit = pair[0].parse::<u32>().unwrap();

                match color {
                    "red" => {
                        if digit > colors[0] {
                            colors[0] = digit;
                        }
                    }
                    "green" => {
                        if digit > colors[1] {
                            colors[1] = digit;
                        }
                    }
                    "blue" => {
                        if digit > colors[2] {
                            colors[2] = digit;
                        }
                    }
                    _ => println!("unknown color"),
                }
            }
        }
        power_cubes += colors.iter().product::<u32>();
    }

    return power_cubes;
}

fn part_one(lines: &Vec<String>) -> usize {
    let possible = lines
        .iter()
        .map(|line| {
            let draws = line.split(";").collect::<Vec<&str>>();
            for draw in draws {
                let mut colors: [u32; 3] = [0; 3];
                let draw = draw.trim();
                let numbers = draw.split(",").collect::<Vec<&str>>();
                for number in numbers {
                    let number = number.trim();
                    let pair = number.split(" ").collect::<Vec<&str>>();
                    let color = pair[1];
                    let digit = pair[0];

                    match color {
                        "red" => colors[0] += digit.parse::<u32>().unwrap(),
                        "green" => colors[1] += digit.parse::<u32>().unwrap(),
                        "blue" => colors[2] += digit.parse::<u32>().unwrap(),
                        _ => println!("unknown color"),
                    }
                }
                if colors[0] > RED_LIMIT {
                    return false;
                }
                if colors[1] > GREEN_LIMIT {
                    return false;
                }
                if colors[2] > BLUE_LIMIT {
                    return false;
                }
            }
            return true;
        })
        .collect::<Vec<bool>>();

    let mut id_sum = 0;
    for (i, game) in possible.iter().enumerate() {
        let id = i + 1;

        if *game {
            id_sum += id;
        }
    }
    return id_sum;
}
