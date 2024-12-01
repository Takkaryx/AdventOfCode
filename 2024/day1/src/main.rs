use std::{cmp::Ordering, fs};

fn parse_input(f: String) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(f).unwrap();
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];
    for line in contents.lines() {
        let mut nums = line.split_whitespace();
        l1.push(nums.next().unwrap().parse().unwrap());
        l2.push(nums.next().unwrap().parse().unwrap());
    }
    (l1, l2)
}

fn solution1(mut l1: Vec<i32>, mut l2: Vec<i32>) -> i32 {
    l1.sort();
    l2.sort();

    let mut ans_vec: Vec<i32> = Vec::new();
    for i in 0..l1.len() {
        match l1[i].cmp(&l2[i]) {
            Ordering::Less => ans_vec.push(l2[i] - l1[i]),
            Ordering::Greater => ans_vec.push(l1[i] - l2[i]),
            Ordering::Equal => ans_vec.push(0),
        }
    }
    ans_vec.into_iter().sum()
}

fn solution2(l1: Vec<i32>, l2: Vec<i32>) -> i32 {
    let mut ans_vec: Vec<i32> = Vec::new();

    for val in &l1 {
        let mut count = 0;
        for val2 in &l2 {
            if *val == *val2 {
                count += 1;
            }
        }
        ans_vec.push(val * count);
    }
    ans_vec.into_iter().sum()
}

fn main() {
    let (l1, l2) = parse_input(String::from("example.txt"));
    println!("{:?}", solution1(l1.clone(), l2.clone()));
    println!("{:?}", solution2(l1.clone(), l2.clone()));
    let (l1, l2) = parse_input(String::from("input.txt"));
    println!("{:?}", solution1(l1.clone(), l2.clone()));
    println!("{:?}", solution2(l1.clone(), l2.clone()));
}
