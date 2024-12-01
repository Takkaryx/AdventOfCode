use std::fs;

fn main() {
    let mut total_score = 0;
    let contents = fs::read_to_string("input.txt").unwrap();
    for line in contents.lines() {
        total_score += get_result(line);
    }
    println!("final result of entire input: {}", total_score);
}

static ROCK: u64 = 1;
static PAPER: u64 = 2;
static SCI: u64 = 3;

static WIN: u64 = 6;
static LOSE: u64 = 0;
static TIE: u64 = 3;

fn get_result(guide: &str) -> u64 {
    let them: char = guide.chars().nth(0).unwrap();
    let us: char = guide.chars().nth(2).unwrap();
    match them {
        'A' => compare_rock(us),
        'B' => compare_paper(us),
        'C' => compare_scissors(us),
        _ => {
            println!("WHAT?");
            0
        }
    }
}

// X need to lose
// Y need to tie
// Z need to win

fn compare_rock(val: char) -> u64 {
    match val {
        'X' => LOSE + SCI,  //rock vs rock
        'Y' => TIE + ROCK,  //rock vs paper
        'Z' => WIN + PAPER, //rock vs sci
        _ => {
            println!("WHAT?");
            0
        }
    }
}

fn compare_paper(val: char) -> u64 {
    match val {
        'X' => LOSE + ROCK, //paper vs paper
        'Y' => TIE + PAPER, //paper vs paper
        'Z' => WIN + SCI,   //paper vs sci
        _ => {
            println!("WHAT?");
            0
        }
    }
}
fn compare_scissors(val: char) -> u64 {
    match val {
        'X' => LOSE + PAPER, //rock vs rock
        'Y' => TIE + SCI,    //rock vs paper
        'Z' => WIN + ROCK,   //rock vs sci
        _ => {
            println!("WHAT?");
            0
        }
    }
}
