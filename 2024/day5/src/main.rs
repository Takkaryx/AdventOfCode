use std::cmp::Ordering;
use std::{collections::HashSet, fs::read_to_string};

type Orders = HashSet<(u32, u32)>;
type Updates = Vec<Vec<u32>>;

fn parse_input(f: &str) -> (Orders, Updates) {
    let mut orders: Orders = HashSet::new();
    let mut updates: Updates = vec![];
    let mut swap_input = false;
    let contents = read_to_string(f).unwrap();
    for line in contents.lines() {
        if line == "" {
            swap_input = true;
            continue;
        }
        match swap_input {
            true => {
                updates.push(line.split(',').map(|i| i.parse::<u32>().unwrap()).collect());
            }
            false => {
                let mut vals = line.split('|');
                let start = vals.next().unwrap().parse::<u32>().unwrap();
                let next = vals.next().unwrap().parse::<u32>().unwrap();
                orders.insert((start, next));
            }
        }
    }
    (orders, updates)
}

fn follows_rule(o: &Orders, u: &Vec<u32>) -> bool {
    o.iter().all(|&(left, right)| {
        let l = u.iter().position(|&x| x == left);
        let r = u.iter().position(|&x| x == right);
        match (l, r) {
            (Some(ls), Some(rs)) => ls <= rs,
            _ => true,
        }
    })
}

fn solution1(o: &Orders, u: &Updates) -> u32 {
    let mut sum = 0;
    for level in u {
        if follows_rule(&o, &level) {
            sum += level[level.len() / 2];
        }
    }
    sum
}

fn solution2(o: &Orders, u: &mut Updates) -> u32 {
    let mut sum = 0;
    for level in u {
        if follows_rule(&o, &level) {
            continue;
        } else {
            level.sort_by(|&a, &b| {
                if o.contains(&(a, b)) {
                    Ordering::Less
                } else if o.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
        }
        sum += level[level.len() / 2];
    }
    sum
}

fn main() {
    let (orders, mut updates) = parse_input("input.txt");
    println!("The sum of middles is: {:?}", solution1(&orders, &updates));
    println!(
        "The sum of middles is: {:?}",
        solution2(&orders, &mut updates)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let (orders, updates) = parse_input("example.txt");
        assert_eq!(solution1(&orders, &updates), 143);
    }
    #[test]
    fn test_2() {
        let (orders, mut updates) = parse_input("example.txt");
        assert_eq!(solution2(&orders, &mut updates), 123);
    }
}
