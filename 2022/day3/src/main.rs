use std::fs;

fn main() {
    let mut total_score = 0;
    let fcontents = fs::read_to_string("input2.txt").unwrap();
    let mut contents = fcontents.lines();
    while let (Some(a), Some(b), Some(c)) = (contents.next(), contents.next(), contents.next()) {
        total_score += get_result(a, b, c);
        println!("a: {}, b: {}, c: {}", a, b, c);
        println!("accumulating: {}", total_score);
    }

    println!("final result of entire input: {}", total_score);
}

fn get_result(val: &str, val1: &str, val2: &str) -> u64 {
    for letter in val.chars() {
        if val1.contains(letter) && val2.contains(letter) {
            return calc_value(letter);
        }
    }
    0
}

fn calc_value(val: char) -> u64 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .into_iter();
    let mut count = 1;
    for letter in alphabet {
        if val == letter {
            return count;
        }
        count = count + 1;
    }
    0
}
