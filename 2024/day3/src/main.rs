use regex::Regex;
use std::fs::read_to_string;

fn solution1(f: String) -> u32 {
    let contents = read_to_string(f).unwrap();
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut sum = 0;
    for (_, [num1, num2]) in re.captures_iter(&contents).map(|c| c.extract()) {
        sum += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
    }
    sum
}

fn solution2(f: String) -> u32 {
    let contents = read_to_string(f).unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;

    re.captures_iter(&contents)
        .filter_map(|cap| {
            if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                if enabled {
                    let x = x.as_str().parse::<u32>().unwrap();
                    let y = y.as_str().parse::<u32>().unwrap();
                    Some(x * y)
                } else {
                    None
                }
            } else {
                match &cap[0] {
                    "don't()" => enabled = false,
                    "do()" => enabled = true,
                    _ => {}
                }
                None
            }
        })
        .sum::<u32>()
}

fn main() {
    dbg!(solution1(String::from("example.txt")));
    dbg!(solution1(String::from("input.txt")));
    dbg!(solution2(String::from("example.txt")));
    dbg!(solution2(String::from("input.txt")));
}
