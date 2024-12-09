use std::{
    collections::{HashMap, HashSet},
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
    let height = rows.len() as i32;
    let mut width = 0;
    let mut signal_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut anti_locations: HashSet<(i32, i32)> = HashSet::new();

    for (row_index, row) in rows.iter().enumerate() {
        width = row.len() as i32;
        for (column_index, signal) in row.chars().enumerate() {
            if signal != '.' {
                let current = signal_map.get_mut(&signal);
                let location = (row_index as i32, column_index as i32);
                match current {
                    Some(vec) => {
                        vec.push(location);
                    }
                    None => {
                        let vec = vec![location];
                        signal_map.insert(signal, vec);
                    }
                }
            }
        }
    }

    let mut total = 0;
    for (signal, locations) in signal_map.iter() {
        for location_one in locations {
            for location_two in locations {
                if location_one != location_two {
                    let height_diff = location_one.0 - location_two.0;
                    let width_diff = location_one.1 - location_two.1;

                    let mut anti_one = (location_one.0 + height_diff, location_one.1 + width_diff);
                    while is_in_grid(height, width, anti_one) {
                        if anti_locations.insert(anti_one) {
                            total += 1;
                            println!("{signal} is in grid at {anti_one:?}");
                        }
                        anti_one = (anti_one.0 + height_diff, anti_one.1 + width_diff);
                    }

                    let mut anti_two = (location_two.0 + height_diff, location_two.1 + width_diff);
                    while is_in_grid(height, width, anti_two) {
                        if anti_locations.insert(anti_two) {
                            total += 1;
                            println!("{signal} is in grid at {anti_two:?}");
                        }
                        anti_two = (anti_two.0 + height_diff, anti_two.1 + width_diff);
                    }
                }
            }
        }
    }
    println!("Total {total}");
}

fn is_in_grid(height: i32, width: i32, location: (i32, i32)) -> bool {
    location.0 >= 0 && location.0 < height && location.1 >= 0 && location.1 < width
}
