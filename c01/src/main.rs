use std::{collections::{BinaryHeap, HashMap}, fs::File, io::Read};

fn read() -> Vec<String> {
    let mut file = File::open("input.txt").expect("No file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file");
    return contents.split('\n')
        .map(|x| x.to_string())
        .filter(|x| x.len() > 0)
        .collect();
}

fn main() {
    let rows = read();
    let mut left: BinaryHeap<i32> = BinaryHeap::new();
    let mut right: BinaryHeap<i32> = BinaryHeap::new();
    let mut right_counts: HashMap<i32, i32> = HashMap::new(); // Vec is probably faster

    for row in rows {
        let split: Vec<i32> = row.split_whitespace()
            .map(|x| x.parse().expect("Parse failed"))
            .collect();
        left.push(split[0]);
        right.push(split[1]);
        
        match right_counts.get(&split[1]) {
            None => right_counts.insert(split[1], 1),
            Some(current) => right_counts.insert(split[1], current + 1)
        };
    }
   
    let mut total = 0;
    let mut similarity = 0;
    while !left.is_empty() {
        let l = left.pop().expect("Pop failed");
        let r = right.pop().expect("Pop failed");

        let diff: i32 = (l - r).abs();
        total = total + diff;

        let count = right_counts.get(&l);
        similarity = similarity + (count.unwrap_or(&0) * l);
    }

    println!("Total {}", total);
    println!("Similarity {}", similarity);
}
