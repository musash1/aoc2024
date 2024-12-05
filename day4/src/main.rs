use std::fs;

fn main() {
    let file = fs::read_to_string("text.txt").expect("Not able to read File.");
    let lines: Vec<String> = file.split("\n").map(|s| s.to_string()).collect();

    println!("Part 1: {:?}", part1(&lines));
    println!("Part 2: {:?}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32 {
    for line in lines {
        for (i, char) in line.chars().enumerate() {
            if char.eq(&'X') {
                let mut block: Vec<String> = Vec::new();
                for j in i..(i + 3) {
                    if j > lines.len() - 3 || j <= 2 || i > line.len() - 3 || i <= 2 {
                        continue;
                    }
                    block.push(lines[j - 3][(i - 3)..(i + 3)].to_string()); 
                }
                println!("{:?}", block);
            }
        }
    }
    0 
}

fn part2(lines: &Vec<String>) -> i32 {
    0
}
