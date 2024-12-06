use std::{collections::HashMap, fs::File, io::Read};

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

    let mut precendences: Vec<(i32, i32)> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];

    for row in rows {
        if row.contains('|') {
            let split: Vec<i32> = row
                .split('|')
                .map(|x| x.parse().expect("Parse failed"))
                .collect();
            precendences.push((split[0], split[1]));
        } else {
            let split: Vec<i32> = row
                .split(',')
                .map(|x| x.parse().expect("Parse failed"))
                .collect();
            updates.push(split);
        }
    }

    let mut total = 0;
    for mut update in updates {
        let mut valid = false;
        let mut sorted = false;
        while !valid {
            valid = true;
            let mut indices: HashMap<i32, usize> = HashMap::new();
            for (index, page) in update.iter().enumerate() {
                indices.insert(*page, index);
            }
            for (before, after) in &precendences {
                if let (Some(before_index), Some(after_index)) =
                    (indices.get(before), indices.get(after))
                {
                    if *before_index >= *after_index {
                        valid = false;
                        sorted = true;
                        update.swap(*before_index, *after_index);
                        break;
                    }
                }
            }
        }
        if sorted {
            total += update[update.len() / 2];
        }
    }

    println!("Total {total}")
}
