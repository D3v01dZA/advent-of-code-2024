use std::{fs::File, io::Read};

fn read() -> Vec<String> {
    let mut file = File::open("input.txt").expect("No file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file");
    return contents
        .split('\n')
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect();
}

fn main() {
    let rows = read();

    let mut rocks: Vec<i64> = rows[0]
        .split_whitespace()
        .map(|x| x.parse().expect("Parse failed"))
        .collect();

    for _ in 0..25 {
        let mut new_rocks = vec![];
        for rock in rocks {
            if rock == 0 {
                new_rocks.push(1);
                continue;
            } 
            let rock_s = format!("{rock}");
            if rock_s.len() > 1 && rock_s.len() % 2 == 0 {
                let (left, right) = rock_s.split_at(rock_s.len() / 2);
                new_rocks.push(left.parse().expect("Couldn't parse left"));
                new_rocks.push(right.parse().expect("Couldn't parse right"));
                continue;
            }
            new_rocks.push(rock * 2024);
        }
        rocks = new_rocks;
    }

    println!("Total {}", rocks.len());
}
