use std::{char, fs::File, io::Read};

fn read() -> Vec<Vec<char>> {
    let mut file = File::open("input.txt").expect("No file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file");
    return contents
        .split('\n')
        .map(|x| x.chars().collect())
        .filter(|x: &Vec<char>| !x.is_empty())
        .collect();
}

fn main() {
    let rows = read();
    let height: i32 = rows.len() as i32;
    let width: i32 = rows[0].len() as i32;
    let mut total = 0;

    for i in 0..height {
        for j in 0..width {
            if rows[i as usize][j as usize] == 'A' {
                if is_mas(&rows, height, width, i, j) {
                    total += 1;
                }
            }
        }
    }

    println!("Total {total}");
}

fn is_mas(rows: &[Vec<char>], height: i32, width: i32, height_loc: i32, width_loc: i32) -> bool {
    let mut left: Vec<char> = vec![];
    let mut right: Vec<char> = vec![];
    let mutated = mutation(rows, left, height, width, height_loc, width_loc, 1, 1);
    if mutated.is_none() {
        return false;
    } else {
        left = mutated.unwrap();
    }
    left.push('A');
    let mutated = mutation(rows, left, height, width, height_loc, width_loc, -1, -1);
    if mutated.is_none() {
        return false;
    } else {
        left = mutated.unwrap();
    }
    let mutated = mutation(rows, right, height, width, height_loc, width_loc, -1, 1);
    if mutated.is_none() {
        return false;
    } else {
        right = mutated.unwrap();
    }
    right.push('A');
    let mutated = mutation(rows, right, height, width, height_loc, width_loc, 1, -1);
    if mutated.is_none() {
        return false;
    } else {
        right = mutated.unwrap();
    }
    let str: String = left.iter().collect();
    if str != "MAS" {
        left.reverse();
        let str: String = left.iter().collect();
        if str != "MAS" {
            return false;
        }
    }
    let str: String = right.iter().collect();
    if str != "MAS" {
        right.reverse();
        let str: String = right.iter().collect();
        if str != "MAS" {
            return false;
        }
    }
    true
}

// This sucks so hard the previous solution was so clean but the borrow checker got mad at me then
// me at it, you could do a for (vec, height_mod, width_mod) in [[vec, 1, 1], vec[-1, -1]] etc if
// it would just let you
fn mutation(
    rows: &[Vec<char>],
    mut vec: Vec<char>,
    height: i32,
    width: i32,
    height_loc: i32,
    width_loc: i32,
    height_mod: i32,
    width_mod: i32,
) -> Option<Vec<char>> {
    let h: i32 = height_loc + height_mod;
    let w: i32 = width_loc + width_mod;
    if h >= height || w >= width || h < 0 || w < 0 {
        return None;
    }
    vec.push(rows[h as usize][w as usize]);
    Some(vec)
}
