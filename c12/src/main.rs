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

    let mut considered: HashSet<(usize, usize)> = HashSet::new();
    let mut calculateds: Vec<(char, u32, u32)> = vec![];

    for (r_index, r) in rows.iter().enumerate() {
        for (c_index, c) in r.chars().enumerate() {
            if !considered.contains(&(r_index, c_index)) {
                let (area, perimeter) = calculate(&rows, &mut considered, r_index, c_index, c);
                calculateds.push((c, area, perimeter));
            }
        }
    }

    let mut total = 0;
    for calculated in calculateds {
        total += calculated.1 * calculated.2;
    }
    println!("Total {total}");
}

fn calculate(
    rows: &Vec<String>,
    considered: &mut HashSet<(usize, usize)>,
    r_index: usize,
    c_index: usize,
    c: char,
) -> (u32, u32) {
    if !considered.insert((r_index, c_index)) {
        return (0, 0);
    }

    let mut current_area = 1;

    let mut current_perimeter = 0;
    if r_index == 0 || rows[r_index - 1].chars().nth(c_index) != Some(c) {
        current_perimeter += 1;
    } else {
        let (area, perimeter) = calculate(rows, considered, r_index - 1, c_index, c);
        current_area += area;
        current_perimeter += perimeter;
    }
    if r_index == rows.len() - 1 || rows[r_index + 1].chars().nth(c_index) != Some(c) {
        current_perimeter += 1;
    } else {
        let (area, perimeter) = calculate(rows, considered, r_index + 1, c_index, c);
        current_area += area;
        current_perimeter += perimeter;
    }
    if c_index == 0 || rows[r_index].chars().nth(c_index - 1) != Some(c) {
        current_perimeter += 1;
    } else {
        let (area, perimeter) = calculate(rows, considered, r_index, c_index - 1, c);
        current_area += area;
        current_perimeter += perimeter;
    }
    if c_index == rows[r_index].len() - 1 || rows[r_index].chars().nth(c_index + 1) != Some(c) {
        current_perimeter += 1;
    } else {
        let (area, perimeter) = calculate(rows, considered, r_index, c_index + 1, c);
        current_area += area;
        current_perimeter += perimeter;
    }
    (current_area, current_perimeter)
}
