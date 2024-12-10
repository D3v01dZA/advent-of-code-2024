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
    let mut blocks: Vec<Option<i64>> = vec![];
    let mut free = false;
    let mut current = 0;

    for c in rows[0].chars() {
        let count: i64 = format!("{c}").parse().expect("Couldn't parse"); // Why no char parse???
        if free {
            for _ in 0..count {
                blocks.push(None);
            }
        } else {
            for _ in 0..count {
                blocks.push(Some(current));
            }
            current += 1;
        }
        free = !free;
    }

    let mut free_index = 0;
    let mut block_index = blocks.len() - 1;

    while free_index < block_index {
        if blocks[free_index].is_none() && blocks[block_index].is_some() {
            blocks[free_index] = blocks[block_index];
            blocks[block_index] = None
        }
        if blocks[free_index].is_some() {
            free_index += 1;
        }
        if blocks[block_index].is_none() {
            block_index -= 1;
        }
    }

    let mut total: i64 = 0;
    for (index, block) in blocks.iter().enumerate() {
        if let Some(block) = block {
            total += index as i64 * *block;
        }
    }

    println!("Total {total}");
}
