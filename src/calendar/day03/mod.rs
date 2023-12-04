use crate::calendar::load_input;
use regex::Regex;

// 1206321 too high
// 1187308 too high
// 1187308
// 514969

pub fn run() {
    println!("Day 03");

    let mut lines = load_input(3)
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            format!("{}{}{}", ".", line, ".")
                .chars()
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    // add lines to top and bottom filled with dots to avoid checking for out of bounds
    let width = lines[0].len();
    let mut top_bottom = String::new();
    for _ in 0..width {
        top_bottom.push('.');
    }
    lines.insert(0, top_bottom.clone());
    lines.push(top_bottom);

    part_one(&lines);
}

#[derive(Clone, Debug)]
struct DigitInfo {
    index: (usize, usize),
    length: usize,
    value: u32,
    enabled: bool,
}

fn part_one(lines: &Vec<String>) -> u32 {
    let re = Regex::new(r"[0-9.]").unwrap();

    let height = lines.len();
    let width = lines[0].len();

    let mut numbers: Vec<DigitInfo> = vec![];
    let mut digit_info = DigitInfo {
        index: (0, 0),
        length: 0,
        value: 0,
        enabled: false,
    };
    let mut active = false;

    println!("{:?}", lines);

    for y in 1..height - 1 {
        for x in 1..width {
            let c = lines[y].chars().nth(x).unwrap();
            if c.is_digit(10) {
                if !active {
                    digit_info.index = (x, y);
                    digit_info.length = 1;
                    digit_info.value = c.to_digit(10).unwrap();
                    digit_info.enabled = false;

                    active = true;
                } else {
                    digit_info.length += 1;
                    digit_info.value = (digit_info.value * 10) + c.to_digit(10).unwrap();
                }

                for j in x - 1..x + 2 {
                    for i in y - 1..y + 2 {
                        // check if char is a digit or "."
                        // let c = lines[i].chars().nth(j).unwrap();
                        // if !re.is_match(&c.to_string()) {
                        //     digit_info.enabled = true;
                        // }
                        if let Some(value) = lines[i].chars().nth(j) {
                            if !value.is_numeric() && value != '.' {
                                digit_info.enabled = true;
                            }
                        }
                    }
                }
            } else {
                if active {
                    numbers.push(digit_info.clone());
                    active = false;
                }
            }
        }
    }
    println!("{:?}", numbers);

    let mut sum = 0;
    for number in numbers {
        if number.enabled {
            sum += number.value;
        }
    }

    println!("Part One: {}", sum);

    sum
}

fn part_two(lines: Vec<String>) -> u32 {
    0
}

// test
#[cfg(test)]
mod tests {
    use crate::calendar::day03::part_one;

    const TEST_DATA: [&str; 10] = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    #[test]
    fn test_part_one() {
        let mut lines = TEST_DATA
            .iter()
            .map(|line| {
                format!("{}{}{}", ".", line, ".")
                    .chars()
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        // add lines to top and bottom filled with dots to avoid checking for out of bounds
        let width = lines[0].len();
        let mut top_bottom = String::new();
        for _ in 0..width {
            top_bottom.push('.');
        }
        lines.insert(0, top_bottom.clone());
        lines.push(top_bottom);

        // println!("{:?}", lines);

        assert_eq!(part_one(&lines), 4361);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(vec![String::from("1, 2, 3; 4, 5, 6; 7, 8, 9")]), 6);
    //     );
    // }
}
