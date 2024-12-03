use std::{fs::File, io::Read};

fn read() -> Vec<String> {
    let mut file = File::open("input.txt").expect("No file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file");
    return contents.split('\n')
        .map(|x| x.to_string())
        .filter(|x| x.len() > 0)
        .collect();
}

fn main() {
    let rows = read();
    
    let mut total = 0;
    let mut left: Vec<char> = vec![];
    let mut right: Vec<char> = vec![];
    let mut progress = 0;
    let mut enabled = true;
    let mut enabled_progress = 0;
    for row in rows {
        for c in row.chars() {
            match progress {
                0 => if c == 'm' { progress += 1 } else { progress = 0 },
                1 => if c == 'u' { progress += 1 } else { progress = 0 },
                2 => if c == 'l' { progress += 1 } else { progress = 0 },
                3 => if c == '(' { progress += 1 } else { progress = 0 },
                4 => if c.is_digit(10) { 
                    progress += 1;
                    left.push(c);
                } else {
                    progress = 0;
                    left.clear();
                },
                5 => if c.is_digit(10) {
                    left.push(c);
                } else if c == ',' {
                    progress += 1;
                } else {
                    progress = 0;
                    left.clear();
                },
                6 => if c.is_digit(10) {
                    right.push(c);
                } else if c == ')' {
                    progress = 0;
                    let l: String = left.clone().into_iter().collect();
                    let r: String = right.clone().into_iter().collect();
                    left.clear();
                    right.clear();
                    let l: i32 = l.parse().expect("Parse failed");
                    let r: i32 = r.parse().expect("Parse failed");
                    if enabled {
                        total += l * r;
                    }
                } else if c == 'm' {
                    progress = 1;
                    left.clear();
                    right.clear();
                } else {
                    progress = 0;
                    left.clear();
                    right.clear();
                },
                _ => {},
            };

            if !enabled {
                match enabled_progress {
                    0 => if c == 'd' { enabled_progress += 1 } else { enabled_progress = 0 },
                    1 => if c == 'o' { enabled_progress += 1 } else { enabled_progress = 0 },
                    2 => if c == '(' { enabled_progress += 1 } else { enabled_progress = 0 },
                    3 => if c == ')' { 
                        enabled = true;
                        enabled_progress = 0;
                    } else { 
                        enabled_progress = 0
                    },
                    _ => {},
                }
            } else {
                match enabled_progress {
                    0 => if c == 'd' { enabled_progress += 1 } else { enabled_progress = 0 },
                    1 => if c == 'o' { enabled_progress += 1 } else { enabled_progress = 0 },
                    2 => if c == 'n' { enabled_progress += 1 } else { enabled_progress = 0 },
                    3 => if c == '\'' { enabled_progress += 1 } else { enabled_progress = 0 },
                    4 => if c == 't' { enabled_progress += 1 } else { enabled_progress = 0 },
                    5 => if c == '(' { enabled_progress += 1 } else { enabled_progress = 0 },
                    6 => if c == ')' { 
                        enabled = false;
                        enabled_progress = 0;
                    } else { 
                        enabled_progress = 0
                    },
                    _ => {},
                }
            }
        }
    }
    println!("Total {}", total);
}
