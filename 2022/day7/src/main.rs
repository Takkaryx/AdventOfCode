use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

mod dir;
use dir::Cfiles;

lazy_static! {
    static ref RE_DIR: Regex = Regex::new(r"dir (?P<name>)").expect("RE_DIR compiles");
    static ref RE_FILE: Regex =
        Regex::new(r"(?P<size>[0-MAX(f64)]) (?P<name>)").expect("RE_FILE compiles");
    static ref RE_LS: Regex = Regex::new(r"\$ ls").expect("RE_LS compiles");
    static ref RE_CD: Regex = Regex::new(r"\$ cd (?P<name>)").expect("RE_CD compiles");
}
fn main() {
    let content = fs::read_to_string("./example.txt").expect("Read ./example.txt");
    build_database(content);
}

fn handle_dir(line: &str) -> String {
    let name: String = RE_DIR
        .captures(line)
        .unwrap()
        .name("name")
        .expect("regex name")
        .as_str()
        .parse()
        .expect("unable to parse");
    name
}

fn handle_cd<'a>(dir: &'a Cdir, line: &'a str) -> Result<&'a mut Cdir, i32> {
    let name: String = RE_CD
        .captures(line)
        .unwrap()
        .name("name")
        .expect("regex name")
        .as_str()
        .parse()
        .expect("unable to parse");
    println!("CD LINE, NAME: {}", name);
    for &mut cdir in dir.subdirs.clone().into_iter() {
        if cdir.name == name {
            return Ok(&cdir);
        }
        if name == "..".to_string() {
            return Ok(cdir.parent.unwrap().as_ref());
        }
    }
    return Err(10);
}

fn build_database(content: String) {
    let line_iter = content.lines();
    let mut root_dir: &mut Cdir;
    let mut cur_dir: &mut Cdir = &mut Cdir::new("".to_string());
    let mut first = true;
    for line in line_iter {
        match line {
            line if RE_DIR.is_match(&line) => {
                let name = handle_dir(line).to_string();
                println!("DIR LINE, NAME: {}", name);
                let dir = Cdir::new(name);
                cur_dir.subdirs.push(dir);
            }
            line if RE_FILE.is_match(&line) => {
                println!("FILE_LINE {}", line);
            }
            line if RE_LS.is_match(&line) => {
                println!("LS_LINE {}", line);
            }
            line if RE_CD.is_match(&line) => {
                if first {
                    let root_name = RE_CD
                        .captures(line)
                        .unwrap()
                        .name("name")
                        .expect("regex name")
                        .as_str()
                        .parse()
                        .expect("unable to parse");
                    println!("CD ROOT LINE, NAME: {}", root_name);
                    root_dir = &mut Cdir::new(root_name);
                    cur_dir = root_dir;
                    first = false;
                } else {
                    cur_dir = handle_cd(cur_dir, line).unwrap();
                }
            }
            _ => {
                println!("invalid line found? {}", line);
            }
        }
    }
}
