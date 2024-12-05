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

fn main() {
    let rows = read();
    
    for row in rows {
        let split: Vec<i32> = row.split_whitespace()
            .map(|x| x.parse().expect("Parse failed"))
            .collect();
    }
}
