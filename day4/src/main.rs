use std::{fs, str::LinesAny};

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
    direction: Direction
}

impl Default for XmasDirection {
    fn default() -> XmasDirection {
        XmasDirection {
            pos: (0, 0),
            direction: Direction::Center
        }
    }
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
                let direction: (i32, i32) = match xmas_direction.direction {
                    Direction::Up => (-1, 0),
                    Direction::UpLeft => (-1, -1),
                    Direction::Left => (0, -1),
                    Direction::DownLeft => (1, -1),
                    Direction::Down => (1, 0),
                    Direction::DownRight => (1, 1),
                    Direction::Right => (0, 1),
                    Direction::UpRight => (-1, 1),
                    Direction::Center => continue
                };

                if lines.get((xmas_direction.pos.0 as i32 + direction.0) as usize).unwrap().chars().nth((xmas_direction.pos.1 as i32 + direction.1) as usize).unwrap() == 'A' 
                   && lines.get((xmas_direction.pos.0 as i32 + direction.0 * 2) as usize).unwrap().chars().nth((xmas_direction.pos.1 as i32 + direction.1 * 2) as usize).unwrap() == 'S'{
                    xmas_count += 1;
                }
            }
        }
    }
    xmas_count
}

fn check_for_letter_m(lines: &Vec<String>, i: usize, j: usize) -> Option<XmasDirection> {
    let mut xmas_direction: XmasDirection = XmasDirection::default();

    let mut i_values: Vec<i32> = [-1, 0, 1].to_vec();
    let mut j_values: Vec<i32> = [-1, 0, 1].to_vec();

    let text_length = lines.len();
    let line_length = lines.get(i).unwrap().chars().count();

    let i_range = 4..=(text_length - 5);
    let j_range = 4..=(line_length - 5);
        
    let i_diff: i32 = (text_length - i) as i32;
    let j_diff: i32 = (line_length - j) as i32;



    if !i_range.contains(&i) {
        match i_diff {
            4 => {i_values.remove(2);},
            0..=3 => {
                i_values.remove(1);
                i_values.remove(2);
            }
            ..=0 => {
                i_values.remove(0);
                i_values.remove(1);
            }
            -4 => {i_values.remove(0);}
            _ => ()
        }
    }

    if !j_range.contains(&j) {
        match i_diff {
            4 => {i_values.remove(2);},
            0..=3 => {
                i_values.remove(1);
                i_values.remove(2);
            }
            -3..=0 => {
                i_values.remove(0);
                i_values.remove(1);
            }
            -4 => {i_values.remove(0);}
            _ => ()
        }
    }

    

    match j_diff {
        4 => {j_values.remove(2);},
        0..=3 => {
            j_values.remove(1);
            j_values.remove(2);
        }
        -3..=0 => {
            j_values.remove(0);
            j_values.remove(1);
        }
        -4 => {j_values.remove(0);}
        _ => ()
    }

    for i_val in i_values {
        j_values.clone().into_iter().for_each(|j_val| {
            if lines.get((i as i32 + i_val) as usize).unwrap().chars().nth((j as i32 + j_val) as usize).unwrap() == 'M' {
                let pos1 = i + i_val as usize;
                let pos2 = j + j_val as usize;
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

                println!("i & j: ({},{}) pos1: {}, pos2: {}", i, j, pos1, pos2);
                xmas_direction.direction = direction;
                xmas_direction.pos = (pos1, pos2)
            } 
        });
    }
    Some(xmas_direction)
}

fn part2(lines: &Vec<String>) -> i32 {
    0
}
