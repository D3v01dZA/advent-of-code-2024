use std::{collections::HashMap, fs::File, io::Read, panic};

use regex::Regex;

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

#[derive(Debug)]
enum Op {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct Operation {
    left: String,
    op: Op,
    right: String,
    output: String,
}

fn main() {
    let rows = read();

    let state_reg = Regex::new("^(.+): (\\d+)$").unwrap();
    let action_reg = Regex::new("^(.+) (AND|OR|XOR) (.+) -> (.+)$").unwrap();

    let mut states: HashMap<String, bool> = HashMap::new();
    let mut actions: Vec<Operation> = vec![];

    for row in rows {
        if row.chars().nth(3).unwrap() == ':' {
            let cap = state_reg.captures(row.as_str()).unwrap();
            let val = match &cap[2] {
                "0" => false,
                "1" => true,
                _ => panic!("Uhh"),
            };
            states.insert(cap[1].to_string(), val);
        } else {
            let cap = action_reg.captures(row.as_str()).unwrap();
            let op = match &cap[2] {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => panic!("Huh"),
            };
            actions.push(Operation {
                left: cap[1].to_string(),
                op: op,
                right: cap[3].to_string(),
                output: cap[4].to_string(),
            });
        }
    }

    println!("Actions {actions:?}");
    println!("States {states:?}");

    while !actions.is_empty() {
        let action = actions.remove(0);
        let left = states.get(&action.left);
        let right = states.get(&action.right);
        if let (Some(left), Some(right)) = (left, right) {
            let out = match action.op {
                Op::AND => *left && *right,
                Op::OR => *left || *right,
                Op::XOR => left ^ right,
            };
            states.insert(action.output.clone(), out);
        } else {
            actions.push(action);
        }
    }

    println!("States {states:?}");

    let mut i: u32 = 0;
    let mut total: i64 = 0;
    print!("LSB first ");
    while let Some(state) = states.get(&format!("z{i:02}").to_string()) {
        if *state {
            if i == 0 {
                total += 1;
            } else {
                total += 2_i64.pow(i);
            }
            print!("1");
        } else {
            print!("0");
        }
        i += 1;
    }

    println!();
    println!("Total {total}");
}
