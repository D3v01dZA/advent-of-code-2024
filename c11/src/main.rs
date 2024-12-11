use std::{collections::HashMap, fs::File, io::Read};

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

const ITERATIONS: i32 = 75;

fn main() {
    let rows = read();

    let rocks: Vec<i64> = rows[0]
        .split_whitespace()
        .map(|x| x.parse().expect("Parse failed"))
        .collect();

    let mut cache = HashMap::new();

    let mut total = 0;
    for rock in rocks {
        total += calculate_rock_count(&mut cache, 1, rock);
    }

    println!("Total {}", total);
}

fn calculate_rock_count(cache: &mut HashMap<(i32, i64), i64>, iteration: i32, rock: i64) -> i64 {
    let iterations_left = ITERATIONS - iteration;
    if iterations_left == -1 {
        return 1;
    }
    if let Some(cached) = cache.get(&(iterations_left, rock)) {
        return *cached;
    }
    if rock == 0 {
        let result = calculate_rock_count(cache, iteration + 1, 1);
        cache.insert((iterations_left, rock), result);
        return result;
    }
    let digits = digit_count(rock);
    if digits % 2 == 0 {
        let mid = mid(digits);
        let left = calculate_rock_count(cache, iteration + 1, rock / mid);
        let right = calculate_rock_count(cache, iteration + 1, rock % mid);
        let result = left + right;
        cache.insert((iterations_left, rock), result);
        return result;
    }
    let result = calculate_rock_count(cache, iteration + 1, rock * 2024);
    cache.insert((iterations_left, rock), result);
    result
}

fn digit_count(number: i64) -> u32 {
    1 + number.ilog10()
}

fn mid(digits: u32) -> i64 {
    10_i64.pow(digits / 2)
}
