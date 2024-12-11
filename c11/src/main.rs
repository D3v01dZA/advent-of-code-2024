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

const ITERATIONS: u32 = 25;

fn main() {
    let rows = read();

    let rocks: Vec<i64> = rows[0]
        .split_whitespace()
        .map(|x| x.parse().expect("Parse failed"))
        .collect();

    let mut total = 0;
    for rock in rocks {
        total += calculate_rock_count(1, rock);
    }

    println!("Total {}", total);
}

fn calculate_rock_count(iteration: u32, rock: i64) -> i64 {
    if iteration > ITERATIONS {
        return 1;
    }
    if rock == 0 {
        return calculate_rock_count(iteration + 1, 1);
    }
    let digits = digit_count(rock);
    if digits % 2 == 0 {
        let mid = mid(digits);
        let left = calculate_rock_count(iteration + 1, rock / mid);
        let right = calculate_rock_count(iteration + 1, rock % mid);
        return left + right;
    }
    calculate_rock_count(iteration + 1, rock * 2024)
}

fn digit_count(number: i64) -> u32 {
    1 + number.ilog10()
}

fn mid(digits: u32) -> i64 {
    10_i64.pow(digits / 2)
}
