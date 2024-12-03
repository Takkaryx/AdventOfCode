use std::fs::read_to_string;

fn parse_input(f: String) -> Vec<Vec<i32>> {
    let reports: Vec<Vec<i32>> = read_to_string(f)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .fold(Vec::with_capacity(1000), |mut reports, levels_list| {
            let mut level_vec: Vec<i32> = vec![];
            for level in levels_list {
                level_vec.push(level.parse().unwrap());
            }
            reports.push(level_vec);
            reports
        });
    reports
}

fn is_valid(v: &Vec<i32>) -> bool {
    let start_order = v[0].cmp(&v[1]);
    let mut prev_v = v[0];
    for &num in &v[1..] {
        let abs_dif = prev_v.abs_diff(num);
        let order = prev_v.cmp(&num);
        let gap = abs_dif > 3 || abs_dif < 1;

        if start_order != order || gap {
            return false;
        }
        prev_v = num;
    }
    true
}

fn is_valid_with_fail(v: &Vec<i32>) -> bool {
    if is_valid(&v) {
        return true;
    }
    for i in 0..v.len() {
        let subset: Vec<i32> = v
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, &n)| n)
            .collect();

        if is_valid(&subset) {
            return true;
        }
    }
    false
}

fn solution1(vals: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for levels_list in vals {
        if is_valid(&levels_list) {
            total += 1;
        }
    }
    total
}

fn solution2(vals: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for levels_list in vals {
        if is_valid_with_fail(&levels_list) {
            total += 1;
        }
    }
    total
}

fn main() {
    let reports = parse_input(String::from("example.txt"));
    println!("example solution1 is : {:?}", solution1(reports));
    let reports = parse_input(String::from("input.txt"));
    println!("input solution1 is : {:?}", solution1(reports));
    let reports = parse_input(String::from("example.txt"));
    println!("example solution2 is : {:?}", solution2(reports));
    let reports = parse_input(String::from("input.txt"));
    println!("input solution2 is : {:?}", solution2(reports));
}
