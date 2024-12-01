use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let mut curr_elf_cal = 0;
    let mut calorie_values = BinaryHeap::new();
    let contents = fs::read_to_string("input.txt").unwrap();
    for line in contents.lines() {
        if line.is_empty() {
            calorie_values.push(curr_elf_cal);
            curr_elf_cal = 0;
        } else {
            let cal: u64 = line.parse().unwrap();
            curr_elf_cal += cal;
        }
    }
    let mut max_cals = 0;
    for _ in 0..3 {
        let cal_count = calorie_values.pop().unwrap();
        println!("elf has {}", cal_count);
        max_cals += cal_count;
    }
    println!("the top 3 elves have {} cals between them!", max_cals);
}
