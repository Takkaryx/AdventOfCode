use std::fs;

fn main() {
    let mut count = 0;
    let input = fs::read_to_string("input.txt").unwrap();
    for line in input.lines() {
        let values = parse_line(line);
        if fully_contain(values) == true {
            count = count + 1;
        }
    }
    println!("count of fully included values: {}", count);
}

fn parse_line(val: &str) -> (i32, i32, i32, i32) {
    let f: Vec<&str> = val.split(',').collect();
    let lowside: Vec<&str> = f[0].split('-').collect();
    let highside: Vec<&str> = f[1].split('-').collect();
    let a = lowside[0].parse().unwrap();
    let b = lowside[1].parse().unwrap();
    let c = highside[0].parse().unwrap();
    let d = highside[1].parse().unwrap();
    (a, b, c, d)
}

fn fully_contain(values: (i32, i32, i32, i32)) -> bool {
    let bot_lowside = values.0;
    let top_lowside = values.1;
    let bot_highside = values.2;
    let top_highside = values.3;
    let lowside: Vec<i32> = (bot_lowside..=top_lowside).collect();
    let highside: Vec<i32> = (bot_highside..=top_highside).collect();

    let high_contains = lowside.iter().any(|&x| highside.contains(&x));
    let low_contains = highside.iter().any(|&x| lowside.contains(&x));

    low_contains | high_contains
}
