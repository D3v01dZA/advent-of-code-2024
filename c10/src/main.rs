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

    let mut grid: Vec<Vec<i32>> = vec![];
    let mut trailheads: Vec<(i32, i32)> = vec![];

    for (row_index, row) in rows.iter().enumerate() {
        let split: Vec<i32> = row.chars().enumerate()
            .map(|(column_index, column)| {
                let height = column as i32 - 0x30;
                if height == 0 {
                    trailheads.push((row_index as i32, column_index as i32));
                }
                height
            })
            .collect();
        grid.push(split);
    }

    let mut total = 0;
    for (row, column) in trailheads {
        total += trace_trail(0, row, column, &grid);
    }
    println!("Total {total}");
}

fn trace_trail(level: i32, row: i32, column: i32, grid: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for (row_mod, column_mod) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let row = row + row_mod;
        let column = column + column_mod;
        if row >= 0 && (row as usize) < grid.len() {
            let grid_row = grid.get(row as usize).expect("Row out of bounds");
            if column >= 0 && (column as usize) < grid_row.len() {
                let next_level = grid_row.get(column as usize).expect("Column out of bounds");
                if *next_level == level + 1 {
                    if *next_level == 9 {
                        total += 1;
                    } else {
                        total += trace_trail(level + 1, row, column, grid);
                    }
                }
            }
        }
    }
    total
}
