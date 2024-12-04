use std::fs;

fn main() {
    let file = fs::read_to_string("text.txt").expect("Not able to read File.");
    let lines: Vec<Vec<char>> = file.split("\n").map(|s| s.to_string().chars().collect()).collect();

    println!("Part 1: {:?}", part1(&lines));
    println!("Part 2: {:?}", part2(&lines));
}

fn part1(lines: &Vec<Vec<char>>) -> i32 {
    for line in lines {
        for char in line {
            if char == 'X' {
                
            }
        }
    }
    0 
}

fn part2(lines: &Vec<Vec<char>>) -> i32 {
    0
}
