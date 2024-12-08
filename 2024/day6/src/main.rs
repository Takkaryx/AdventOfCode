use std::fmt;

#[derive(Debug, PartialEq)]
enum Face {
    Up,
    Down,
    Left,
    Right,
}

impl Face {
    fn turn_right(&self) -> Face {
        let new_dir = match self {
            Face::Up => Face::Right,
            Face::Down => Face::Left,
            Face::Right => Face::Down,
            Face::Left => Face::Up,
        };
        new_dir
    }
}

#[derive(Debug, PartialEq)]
struct Guard {
    row: isize,
    col: isize,
    face: Face,
}

impl Guard {
    fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            face: Face::Up,
        }
    }
}
#[derive(Debug, PartialEq)]
struct Room {
    room: Vec<Vec<u8>>,
    guard: Guard,
}

impl Room {
    fn new(room: Vec<Vec<u8>>) -> Self {
        let mut my_room = Room {
            room,
            guard: Guard::new(),
        };
        my_room.guard = my_room.guard_pos();
        my_room
    }

    fn guard_pos(&self) -> Guard {
        for (row, char_row) in self.room.iter().enumerate() {
            for (col, c) in char_row.iter().enumerate() {
                match *c {
                    b'v' => {
                        return Guard {
                            row: row as isize,
                            col: col as isize,
                            face: Face::Down,
                        }
                    }
                    b'^' => {
                        return Guard {
                            row: row as isize,
                            col: col as isize,
                            face: Face::Up,
                        }
                    }
                    b'>' => {
                        return Guard {
                            row: row as isize,
                            col: col as isize,
                            face: Face::Right,
                        }
                    }
                    b'<' => {
                        return Guard {
                            row: row as isize,
                            col: col as isize,
                            face: Face::Left,
                        }
                    }
                    _ => continue,
                };
            }
        }

        Guard {
            row: 0,
            col: 0,
            face: Face::Down,
        }
    }

    fn obstruction_ahead(&self) -> bool {
        let (row, col) = match self.guard.face {
            Face::Up => (self.guard.row - 1, self.guard.col),
            Face::Down => (self.guard.row + 1, self.guard.col),
            Face::Right => (self.guard.row, self.guard.col + 1),
            Face::Left => (self.guard.row, self.guard.col - 1),
        };

        if self.room[row as usize][col as usize] == b'#' {
            return true;
        }
        false
    }

    fn guard_out(&self) -> bool {
        if self.guard.row < 0
            || self.guard.row >= (self.room[0].len() - 1) as isize
            || self.guard.col < 0
            || self.guard.col >= (self.room.len() - 1) as isize
        {
            return true;
        }
        false
    }

    fn guard_step(&mut self) {
        if self.obstruction_ahead() {
            self.guard.face = self.guard.face.turn_right();
        }
        match self.guard.face {
            Face::Up => self.guard.row -= 1,
            Face::Down => self.guard.row += 1,
            Face::Right => self.guard.col += 1,
            Face::Left => self.guard.col -= 1,
        }
    }

    fn walk(&mut self) -> u32 {
        let mut num_visited = 1;
        self.room[self.guard.row as usize][self.guard.col as usize] = b'X';
        'a: loop {
            // println!("{}", self);
            if self.guard_out() {
                break 'a;
            }
            self.guard_step();
            if self.room[self.guard.row as usize][self.guard.col as usize] == b'X' {
                continue;
            }
            num_visited += 1;
            self.room[self.guard.row as usize][self.guard.col as usize] = b'X';
        }
        // println!("{}", self);
        num_visited
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let guard_char = match self.guard.face {
            Face::Up => '^',
            Face::Down => 'v',
            Face::Right => '>',
            Face::Left => '<',
        };
        for (row, v) in self.room.iter().enumerate() {
            let mut full: Vec<char> = Vec::new();
            for (col, c) in v.iter().enumerate() {
                if row == self.guard.row as usize && col == self.guard.col as usize {
                    full.push(guard_char);
                } else {
                    full.push(char::from(*c))
                }
            }
            writeln!(f, "{:?}", String::from_iter(full.iter()))?;
        }
        Ok(())
    }
}

fn parse_input(f: &str) -> Room {
    let contents = std::fs::read_to_string(f).unwrap();
    let map = contents
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    Room::new(map)
}

fn main() {
    let mut room_map = parse_input("input.txt");
    let locs = room_map.walk();
    println!("number of discrete locations: {}", locs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room() {
        let input = parse_input("example.txt");
        let data = Room {
            room: vec![
                vec![b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'#', b'.', b'.', b'^', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.'],
                vec![b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'.'],
            ],
            guard: Guard::new(),
        };
        assert_eq!(input.room, data.room);
    }

    #[test]
    fn guard_pos() {
        let input = parse_input("example.txt");
        assert_eq!(
            input.guard,
            Guard {
                row: 6,
                col: 4,
                face: Face::Up
            }
        );
    }

    #[test]
    fn walk() {
        let mut input = parse_input("example.txt");
        assert_eq!(input.walk(), 41);
    }
}
