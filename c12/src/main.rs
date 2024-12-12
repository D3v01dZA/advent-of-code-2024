use std::{collections::HashMap, fs::File, io::Read};

fn read() -> Vec<String> {
    let mut file = File::open("input.txt").expect("No file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file");
    return contents.split('\n')
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty()) 
        .collect();
}

fn main() {
    let rows = read();

    let mut areas: HashMap<char, u32> = HashMap::new();
    let mut perimeters: HashMap<char, u32> = HashMap::new();
    
    for (r_index, r) in rows.iter().enumerate() {
        for (c_index, c) in r.chars().enumerate() {
            let current_area = *areas.get(&c).unwrap_or(&0_u32);
            areas.insert(c, current_area + 1);

            let mut current_perimeter = *perimeters.get(&c).unwrap_or(&0_u32);
            if r_index == 0 || rows[r_index - 1].chars().nth(c_index) != Some(c) {
                current_perimeter += 1;
            }
            if r_index == rows.len() - 1 || rows[r_index + 1].chars().nth(c_index) != Some(c) {
                current_perimeter += 1;
            }
            if c_index == 0 || r.chars().nth(c_index - 1) != Some(c) {
                current_perimeter += 1;
            }
            if c_index == r.len() - 1 || r.chars().nth(c_index + 1) != Some(c) {
                current_perimeter += 1;
            }
            perimeters.insert(c, current_perimeter);
        }
    }

    let mut total = 0;
    for (c, area) in areas {
        total += area * *perimeters.get(&c).unwrap();
    }
    println!("Total {total}");
}
