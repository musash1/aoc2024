use std::fs;

use regex::Regex;

fn main() {
    let file = fs::read_to_string("text.txt").expect("Not able to read File.");

    println!("Part 1: {:?}", part1(&file));
    println!("Part 2: {:?}", part2(&file));
}

fn check_mul(input: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\(([+-]?\d+(?:\.\d+)?),([+-]?\d+(?:\.\d+)?)\)").unwrap();
    let mut mul_list: Vec<&str> = Vec::new();

    for mat in re.find_iter(input) {
        mul_list.push(mat.as_str());
    }

    mul_list
}

fn get_comma(input: &str) -> usize {
    for c in input.chars() {
        if c == ',' {
            return input.chars().position(|x| x == c).unwrap();
        }
    };

    0
}

fn part1(input: &str) -> i32 {
    let mut result = 0;

    let checked_input = check_mul(input);

    for mul in checked_input {
        let comma_pos = get_comma(mul);
        result += mul.get(4..comma_pos).unwrap().parse::<i32>().unwrap() * mul.get((comma_pos + 1)..(mul.len() - 1)).unwrap().parse::<i32>().unwrap();
    }

    result
}

fn part2(input: &str) -> i32 {
    0
}
