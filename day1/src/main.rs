use std::fs;


fn main() {
    let file = fs::read_to_string("text.txt").expect("Not able to read File.");
    let lines = file.split("\n");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    
    for s in lines {
        let mut split: Vec<String> = s.split(' ').map(|item| item.to_string()).collect();
        split.retain(|x| !x.is_empty());

        if split.is_empty() {
            break
        };

        left.push(split[0].parse::<i32>().unwrap());
        right.push(split[1].parse::<i32>().unwrap());
    }

    println!("Part 1: {:?}", part1(left.clone(), right.clone()));
    println!("Part 2: {:?}", part2(left.clone(), right.clone()));
}

fn part1(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut distance = 0;
    let mut left = vec1;
    let mut right = vec2;

    for _ in 0..left.len() {
        let l = left.iter().min().unwrap();
        let r = right.iter().min().unwrap();
        distance += (l - r).abs();
        
        left.remove(left.iter().position(|x| x == l).unwrap());
        right.remove(right.iter().position(|x| x == r).unwrap());
    }
    
    distance
}

fn part2(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut similarity = 0;

    for x in left {
        similarity += x * (right.iter().filter(|&&y| y == x).count() as i32);
    }

    similarity
}
