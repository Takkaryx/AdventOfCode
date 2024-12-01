use anyhow::bail;
use colored::Colorize;
use dbg_pls::{color, DebugPls};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt::Display, fs, str::FromStr, vec};

lazy_static! {
    static ref RE_STACK_CRATE: Regex = Regex::new(
        r"(?x)
            (?:
                (?-x:   ) # matches 3 spaces
                | # or
                \[ # matches '[X]' and captures the item inside
                    (?P<item>[A-Z])
                \]
            )
            \ ? # be hungry about the space after (so it doesn't mess up alignment of next capture)
        "
    )
    .expect("RE_STACK_CRATES compiles");
    static ref RE_MOVE: Regex =
        Regex::new(r"move (?P<count>[0-9]+) from (?P<from>[0-9]+) to (?P<to>[0-9]+)")
            .expect("RE_MOVE compiles");
}

#[derive(Clone, Debug, DebugPls)]
struct Stack {
    /// Crates (from bottom)
    crates: Vec<char>,
}
#[derive(DebugPls)]
struct StackSet {
    /// Stacks
    stacks: Vec<Stack>,
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Read ./input.txt");

    let (mut stackset, moves) = parse_input(content);
    println!("Stacks:\n{}", stackset);
    println!("Moves:");
    moves.iter().for_each(|m| println!("{}", m));
    for mov in moves {
        perform_move(mov, &mut stackset);
    }
    let final_string = stackset
        .stacks
        .iter()
        .map(|s| s.crates.last().expect("has last crate"));
    for stack in final_string {
        print!("{}", stack);
    }
    println!("");
}

fn perform_move(mov: Move, stackset: &mut StackSet) {
    let from_stack = &mut stackset.stacks[mov.from - 1].crates;
    let from_height = from_stack.len();
    let mut items = from_stack
        .drain((from_height - mov.count)..)
        .collect::<Vec<_>>();
    assert_eq!(items.len(), mov.count);
    println!(
        "Moving from {} to {}: {:?}",
        mov.from,
        mov.to,
        color(&items)
    );
    stackset.stacks[mov.to - 1].crates.append(&mut items);
}

fn parse_input(content: String) -> (StackSet, Vec<Move>) {
    let mut levels = vec![];
    let mut line_iter = content.lines();
    for line in &mut line_iter {
        if !line.contains('[') {
            break;
        }
        let mut crates: Vec<Option<char>> = vec![];
        for crate_match in RE_STACK_CRATE.captures_iter(line) {
            crates.push(crate_match.name("item").map(|m| {
                m.as_str()
                    .chars()
                    .next()
                    .expect("single char in capture group")
            }));
        }
        println!("'{}' => ({} crates) {:?}", line, crates.len(), crates);
        levels.push(crates);
    }
    //color!(&levels);
    let stack_count = levels[0].len();
    assert!(levels.iter().all(|l| l.len() == stack_count));
    let mut stacks = vec![Stack { crates: vec![] }; stack_count];
    for level in levels.iter().rev() {
        for (index, item) in level.iter().enumerate() {
            //dbg!(item, index);
            if let Some(item) = item {
                stacks[index].crates.push(*item);
            }
        }
    }
    //color!(&stacks);
    assert_eq!(
        line_iter.next().expect("line after crates"),
        "",
        "empty line after stacks"
    ); // empty line
    let mut moves = vec![];
    for line in line_iter {
        moves.push(line.parse().expect("Parse move line"))
    }

    (StackSet { stacks }, moves)
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} crates: {} -> {}",
            self.count, self.from, self.to
        )?;
        Ok(())
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match RE_MOVE.captures(s) {
            Some(captures) => Ok(Move {
                count: captures
                    .name("count")
                    .expect("regex group 'count'")
                    .as_str()
                    .parse()
                    .expect("parse 'count'"),
                from: captures
                    .name("from")
                    .expect("regex group 'from'")
                    .as_str()
                    .parse()
                    .expect("parse 'from'"),
                to: captures
                    .name("to")
                    .expect("regex group 'to'")
                    .as_str()
                    .parse()
                    .expect("parse 'to'"),
            }),
            None => bail!("not matching move regex: '{}'", s),
        }
    }
}

impl Display for StackSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.stacks.iter().map(|s| s.crates.len()).max();
        match height {
            None => write!(f, "No stacks")?,
            Some(height) => {
                for level in (0..height).rev() {
                    writeln!(
                        f,
                        "{}",
                        self.stacks
                            .iter()
                            .map(|stack| {
                                if stack.crates.len() > level {
                                    format!("[{}]", stack.crates[level]).blue()
                                } else {
                                    "   ".to_string().normal()
                                }
                                .to_string()
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                            .as_str(),
                    )?;
                }
            }
        }
        Ok(())
    }
}
