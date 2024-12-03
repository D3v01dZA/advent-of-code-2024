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
    
    let mut safe = 0;
    for row in rows {
        let split: Vec<i32> = row.split_whitespace()
            .map(|x| x.parse().expect("Parse failed"))
            .collect();
        if is_safe(split) {
            safe = safe + 1;
        }
    }

    println!("Safe {}", safe);
}

fn is_safe(reports: Vec<i32>) -> bool {
    let mut safe = true;
    let mut previous: Option<i32> = None;
    let mut increasing: Option<bool> = None;
    for report in reports {
        if previous.is_none() {
            previous = Some(report);
        } else if previous.unwrap() == report {
            safe = false;
            break;
        } else if (previous.unwrap() - report).abs() > 3 {
            safe = false;
            break;
        } else if increasing.is_none() {
            increasing = Some(previous.unwrap() < report);
            previous = Some(report);
        } else if increasing.unwrap() && previous.unwrap() > report  {
            safe = false;
            break;
        } else if !increasing.unwrap() && previous.unwrap() < report  {
            safe = false;
            break;
        } else {
            previous = Some(report);
        }
    }
    return safe;
}
