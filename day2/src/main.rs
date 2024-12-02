use std::fs;

fn main() {
    let file = fs::read_to_string("text.txt").expect("Not able to read File.");
    let lines = file.split("\n");

    let mut input: Vec<Vec<i32>> = Vec::new();

    for s in lines {
        let split: Vec<i32> = s
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|item| item.parse::<i32>().unwrap())
            .collect();

        if split.is_empty() {
            continue;
        }

        input.push(split);
    }

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(input));
}

fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    let mut notation = false;

    'outer: for report in input {
        for i in 0..report.len() {
            if i == report.len() - 1 {
                safe_reports += 1; 
                continue 'outer;
            }

            if report.get(i) == report.get(i + 1) {continue 'outer;}
            if (report.get(i).unwrap() - report.get(i + 1).unwrap()).abs() > 3 {continue 'outer;}

            if i == 0 {
                notation = report.get(i) < report.get(i + 1);
            }

            if (report.get(i) < report.get(i + 1)) != notation {continue 'outer;}
        }
    }

    safe_reports
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    let mut notation = false;

    'outer: for report in input {
        'middle: for i in 0..report.len() {
            let mut temp = report.clone();
            temp.remove(i);
            'inner: for i in 0..report.len() {
                if i == temp.len() - 1 {
                    safe_reports += 1;
                    break 'middle;
                }

                if temp.get(i) == temp.get(i + 1) {break;}
                if (temp.get(i).unwrap() - temp.get(i + 1).unwrap()).abs() > 3 {break;}

                if i == 0 {
                    notation = temp.get(i) < temp.get(i + 1);
                }

                if (temp.get(i) < temp.get(i + 1)) != notation {break;}
            }
        }
    }
    safe_reports
}

