use std::fs;

enum Direction {
    Up, 
    UpRight, 
    Right, 
    DownRight, 
    Down, 
    DownLeft, 
    Left, 
    UpLeft,
    Center
}

struct XmasDirection {
    pos: (usize, usize),
    character: char,
    direction: Direction
}

fn main() {
    let file = fs::read_to_string("test.txt").expect("Not able to read File.");
    let lines: Vec<String> = file.split("\n").map(|s| s.to_string()).collect();

    println!("Part 1: {:?}", part1(&lines));
    println!("Part 2: {:?}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut xmas_count = 0;

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char.eq(&'X') {
                let xmas_direction = match check_for_letter_m(&lines, i, j) {
                    Some(x) => x,
                    None => continue
                };
                
            }
        }
    }
    xmas_count
}

fn check_for_letter_m(lines: &Vec<String>, i: usize, j: usize) -> Option<XmasDirection> {
    let mut i_values: Vec<i32> = [-1, 1].to_vec();
    let mut j_values: Vec<i32>= [-1, 1].to_vec();
    if i >= lines.len() {
        i_values.remove(1);
    } else if i <= 0 {
        i_values.remove(0);
    }

    if j >= lines.get(i).unwrap().chars().count() {
        j_values.remove(1);
    } else if j <= 0 {
        j_values.remove(0);
    }

    for i_val in i_values {
        for j_val in &j_values {
            if lines.get((i as i32 + i_val) as usize).unwrap().chars().nth((j as i32 + j_val) as usize).unwrap() == 'M' {
                let pos1 = i + i_val as usize;
                let pos2 = j + *j_val as usize;
                let direction = match (i_val, j_val) {
                    (-1, 0)  => Direction::Up,
                    (-1, -1) => Direction::UpLeft,
                    (0, -1)  => Direction::Left,
                    (1, -1)  => Direction::DownLeft,
                    (1, 0)   => Direction::Down,
                    (1, 1)   => Direction::DownRight,
                    (0, 1)   => Direction::Right,
                    (-1, 1)  => Direction::UpRight,
                    _        => Direction::Center
                };
                return Some(XmasDirection { 
                    pos: (pos1, pos2), 
                    character: 'M', 
                    direction
                });
            } 
        }
    }
    None
}

fn part2(lines: &Vec<String>) -> i32 {
    0
}
