use std::{fs::File, io::Read};

fn read() -> Vec<String> {
    let mut file = File::open("input.txt").expect("No file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file");
    return contents.split('\n')
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty()) 
        .collect();
}

struct Input {
    result: i64,
    values: Vec<i64>,
}

fn main() {
    let rows = read();

    let mut inputs: Vec<Input> = vec![];
    
    for row in rows {
        let split: Vec<String> = row.split(": ")
            .map(|x| x.to_string())
            .collect();

        let values: Vec<i64> = split[1].split_whitespace()
            .map(|x| x.parse().expect("Could not parse"))
            .collect();
        
        inputs.push(Input { result: split[0].parse().expect("Could not parse result"), values: values });
    }

    let mut total = 0;
    for input in inputs {
        let mut values = input.values;
        let current = values.remove(0);
        if calculate(input.result, current, values) {
            total += input.result;
        }
    }
    println!("Total {total}");
}

fn calculate(expected: i64, current: i64, mut remaining: Vec<i64>) -> bool {
    let next = remaining.remove(0);
    let add_current = current + next;
    let mul_current = current * next;
    if remaining.is_empty() {
        return add_current == expected || mul_current == expected;
    }
    calculate(expected, add_current, remaining.clone()) || calculate(expected, mul_current, remaining.clone())
}
