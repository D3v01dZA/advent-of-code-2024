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
            if rows[i as usize][j as usize] == 'X' {
                // left -> right
                if is_mas(&rows, height, width, i, j, 0, 1) {
                    total += 1;
                }
                // right -> left
                if is_mas(&rows, height, width, i, j, 0, -1) {
                    total += 1;
                }
                // top -> bottom
                if is_mas(&rows, height, width, i, j, 1, 0) {
                    total += 1;
                }
                // bottom -> top
                if is_mas(&rows, height, width, i, j, -1, 0) {
                    total += 1;
                }
                // top left -> bottom right
                if is_mas(&rows, height, width, i, j, 1, 1) {
                    total += 1;
                }
                // top right -> bottom left
                if is_mas(&rows, height, width, i, j, 1, -1) {
                    total += 1;
                }
                // bottom left -> top right
                if is_mas(&rows, height, width, i, j, -1, 1) {
                    total += 1;
                }
                // bottom right -> top left
                if is_mas(&rows, height, width, i, j, -1, -1) {
                    total += 1;
                }
            }
        }
    }

    println!("Total {total}");
}

fn is_mas(
    rows: &[Vec<char>],
    height: i32,
    width: i32,
    height_loc: i32,
    width_loc: i32,
    height_mod: i32,
    width_mod: i32,
) -> bool {
    let mut vec: Vec<char> = vec![];
    for i in 1..4 {
        let h: i32 = height_loc + (height_mod * i);
        let w: i32 = width_loc + (width_mod * i);
        if h >= height || w >= width || h < 0 || w < 0 {
            return false;
        }
        vec.push(rows[h as usize][w as usize]);
    }
    let string: String = vec.iter().collect();
    string == "MAS"
}
