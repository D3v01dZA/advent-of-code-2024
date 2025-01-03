use std::{collections::HashSet, fs::File, io::Read};

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

#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let rows = read();

    let mut location: (i32, i32) = (0, 0);
    let mut obstructions_grid: Vec<Vec<bool>> = vec![];

    for (row_index, row) in rows.iter().enumerate() {
        let mut obstruction_row: Vec<bool> = vec![];
        for (column_index, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    obstruction_row.push(true);
                }
                '^' => {
                    location = (row_index as i32, column_index as i32);
                    obstruction_row.push(false);
                }
                _ => {
                    obstruction_row.push(false);
                }
            }
        }
        obstructions_grid.push(obstruction_row);
    }

    let mut extra = 0;

    for (row_index, row) in obstructions_grid.iter().enumerate() {
        for (column_index, obstacle) in row.iter().enumerate() {
            if !*obstacle {
                let mut location = location;
                let mut obstructions_grid = obstructions_grid.clone();
                let mut direction = Direction::Up;
                let mut locations: HashSet<(i32, i32, Direction)> = HashSet::new();
                locations.insert((location.0, location.1, direction.clone()));
                obstructions_grid[row_index][column_index] = true;

                loop {
                    let new_location = match direction {
                        Direction::Up => (location.0 - 1, location.1),
                        Direction::Down => (location.0 + 1, location.1),
                        Direction::Left => (location.0, location.1 - 1),
                        Direction::Right => (location.0, location.1 + 1),
                    };

                    if new_location.0 < 0 || new_location.0 >= obstructions_grid.len() as i32 {
                        break;
                    }
                    let obstruction_row = &obstructions_grid[new_location.0 as usize];
                    if new_location.1 < 0 || new_location.1 >= obstruction_row.len() as i32 {
                        break;
                    }

                    if obstruction_row[new_location.1 as usize] {
                        direction = match direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        }
                    } else {
                        location = new_location;
                        if !locations.insert((location.0, location.1, direction.clone())) {
                            extra += 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("Extra {extra}");
}
