use std::{
    collections::HashSet,
    fs::File,
    io::Read,
};

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

    let mut considered: HashSet<(i32, i32)> = HashSet::new();
    let mut calculateds: Vec<(char, HashSet<(i32, i32)>)> = vec![];

    for (r_index, r) in rows.iter().enumerate() {
        for (c_index, c) in r.chars().enumerate() {
            if !considered.contains(&(r_index as i32, c_index as i32)) {
                let area = calculate(&rows, &mut considered, r_index as i32, c_index as i32, c);
                calculateds.push((c, area));
            }
        }
    }

    let mut total = 0;
    for calculated in calculateds {
        total += calculated.1.len() as i32 * sides(calculated.1);
    }
    println!("Total {total}");
}

fn calculate(
    rows: &Vec<String>,
    considered: &mut HashSet<(i32, i32)>,
    r_index: i32,
    c_index: i32,
    c: char,
) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    if !considered.insert((r_index, c_index)) {
        return result;
    }
    result.insert((r_index, c_index));
    if r_index != 0 && rows[r_index as usize - 1].chars().nth(c_index as usize) == Some(c) {
        for res in calculate(rows, considered, r_index - 1, c_index, c) {
            result.insert(res);
        }
    }
    if r_index as usize != rows.len() - 1 && rows[r_index as usize + 1].chars().nth(c_index as usize) == Some(c) {
        for res in calculate(rows, considered, r_index + 1, c_index, c) {
            result.insert(res);
        }
    }
    if c_index != 0 && rows[r_index as usize].chars().nth(c_index as usize - 1) == Some(c) {
        for res in calculate(rows, considered, r_index, c_index - 1, c) {
            result.insert(res);
        }
    }
    if c_index as usize != rows[r_index as usize].len() - 1 && rows[r_index as usize].chars().nth(c_index as usize + 1) == Some(c) {
        for res in calculate(rows, considered, r_index, c_index + 1, c) {
            result.insert(res);
        }
    }
    result
}

fn sides(blocks: HashSet<(i32, i32)>) -> i32 {
    let mut sides = 0;
    let mut sides_considered: HashSet<(i8, i32, i32)> = HashSet::new();
    for block in &blocks {
        if is_top(&blocks, block) && !sides_considered.contains(&(0, block.0, block.1)) {
            sides += 1;
            let r = block.0;
            let mut c = block.1 - 1;
            while is_top(&blocks, &(r, c)) {
                sides_considered.insert((0, r, c));
                c -= 1;
            }
            let r = block.0;
            let mut c = block.1 + 1;
            while is_top(&blocks, &(r, c)) {
                sides_considered.insert((0, r, c));
                c += 1;
            }
        }
        if is_bottom(&blocks, block) && !sides_considered.contains(&(1, block.0, block.1)) {
            sides += 1;
            let r = block.0;
            let mut c = block.1 - 1;
            while is_bottom(&blocks, &(r, c)) {
                sides_considered.insert((1, r, c));
                c -= 1;
            }
            let r = block.0;
            let mut c = block.1 + 1;
            while is_bottom(&blocks, &(r, c)) {
                sides_considered.insert((1, r, c));
                c += 1;
            }
        }
        if is_left(&blocks, block) && !sides_considered.contains(&(2, block.0, block.1)) {
            sides += 1;
            let mut r = block.0;
            let c = block.1;
            while is_left(&blocks, &(r, c)) {
                sides_considered.insert((2, r, c));
                r -= 1;
            }
            let mut r = block.0;
            let c = block.1;
            while is_left(&blocks, &(r, c)) {
                sides_considered.insert((2, r, c));
                r += 1;
            }
        }
        if is_right(&blocks, block) && !sides_considered.contains(&(3, block.0, block.1)) {
            sides += 1;
            let mut r = block.0;
            let c = block.1;
            while is_right(&blocks, &(r, c)) {
                sides_considered.insert((3, r, c));
                r -= 1;
            }
            let mut r = block.0;
            let c = block.1;
            while is_right(&blocks, &(r, c)) {
                sides_considered.insert((3, r, c));
                r += 1;
            }
        }
    }
    sides
}

fn is_right(blocks: &HashSet<(i32, i32)>, block: &(i32, i32)) -> bool {
    blocks.contains(block) && !blocks.contains(&(block.0, block.1 + 1))
}

fn is_left(blocks: &HashSet<(i32, i32)>, block: &(i32, i32)) -> bool {
    blocks.contains(block) && !blocks.contains(&(block.0, block.1 - 1))
}

fn is_bottom(blocks: &HashSet<(i32, i32)>, block: &(i32, i32)) -> bool {
    blocks.contains(block) && !blocks.contains(&(block.0 + 1, block.1))
}

fn is_top(blocks: &HashSet<(i32, i32)>, block: &(i32, i32)) -> bool {
    blocks.contains(block) && !blocks.contains(&(block.0 - 1, block.1))
}
