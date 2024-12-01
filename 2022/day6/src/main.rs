use std::collections::HashSet;
use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Read ./input.txt");
    let content_char: Vec<char> = content.chars().collect::<Vec<_>>();
    let mut character = 0;
    for _ in &content_char {
        if character > 13 {
            let slice = &content_char[character - 14..character];
            println!("{:?}", slice);
            if is_marker(&slice) {
                break;
            }
        }
        character += 1;
    }
    println!("{}", character)
}

fn is_marker(mark: &[char]) -> bool {
    let mut seen_chars = HashSet::new();
    for &c in mark {
        if !seen_chars.insert(c) {
            // If we can't insert the character into the set, it means
            // that we have already seen it before.
            return false;
        }
    }
    true
}
