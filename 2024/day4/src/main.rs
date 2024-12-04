use std::{char, fs};

const GREET: [char; 4] = ['X', 'M', 'A', 'S'];

struct GreetingMap {
    map: Vec<Vec<char>>,
}

impl GreetingMap {
    fn new(map: Vec<Vec<char>>) -> Self {
        Self { map }
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        *self.map.get(x).and_then(|row| row.get(y)).unwrap_or(&'_')
    }

    fn find_xmas(&self, x_pos: usize, y_pos: usize) -> usize {
        [
            (0, -1),
            (-1, 0),
            (0, 1),
            (1, 0),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ]
        .iter()
        .filter(|(delta_row, delta_col)| {
            (1..4).all(|i| {
                let (row, col) = (
                    x_pos + (delta_row * i) as usize,
                    y_pos + (delta_col * i) as usize,
                );
                self.get_char(row, col) == GREET[i as usize]
            })
        })
        .count()
    }

    fn find_x_mas(&self, x: usize, y: usize) -> bool {
        let top = [self.get_char(x - 1, y - 1), self.get_char(x + 1, y + 1)];
        let bot = [self.get_char(x - 1, y + 1), self.get_char(x + 1, y - 1)];
        [top, bot]
            .iter()
            .all(|c| *c == ['M', 'S'] || *c == ['S', 'M'])
    }
}

fn parse_input(f: String) -> GreetingMap {
    let mut map: Vec<Vec<char>> = vec![];
    let contents = fs::read_to_string(f).unwrap();
    for line in contents.lines() {
        map.push(line.chars().collect());
    }
    GreetingMap::new(map)
}

fn solution(map: &GreetingMap) -> (usize, usize) {
    let mut sum = 0;
    let mut sum1 = 0;
    for i in 0..map.map.len() {
        for j in 0..map.map[0].len() {
            match map.map[i][j] {
                'X' => sum += map.find_xmas(i, j),
                'A' => sum1 += map.find_x_mas(i, j) as usize,
                _ => {}
            }
        }
    }
    (sum, sum1)
}

fn main() {
    let search = parse_input(String::from("example.txt"));
    dbg!(solution(&search));
    let search = parse_input(String::from("input.txt"));
    dbg!(solution(&search));
}
