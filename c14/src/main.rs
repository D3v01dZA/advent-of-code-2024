use regex::Regex;
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

#[derive(Debug, Clone, Copy)]
struct Robot {
    location: (i32, i32),
    velocity: (i32, i32),
}

fn main() {
    let rows = read();

    let reg = Regex::new("^p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)$").unwrap();
    let mut robots = vec![];

    for row in rows {
        let cap = reg.captures(row.as_str()).unwrap();
        robots.push(Robot {
            location: (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            velocity: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
        });
    }

    let width = 101;
    let height = 103;
    let seconds = 100;

    for robot in &mut robots {
        for _ in 0..seconds {
            let x_added = robot.location.0 + robot.velocity.0;
            let x = if width <= x_added {
                (width - x_added).abs()
            } else if x_added < 0 {
                width + x_added
            } else {
                x_added
            };
            let y_added = robot.location.1 + robot.velocity.1;
            let y = if height <= y_added {
                (height - y_added).abs()
            } else if y_added < 0 {
                height + y_added
            } else {
                y_added
            };
            robot.location = (x, y);
        }
    }


    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    for robot in robots {
        if let Some(current) = map.get(&robot.location) {
            map.insert(robot.location, current + 1);
        } else {
            map.insert(robot.location, 1);
        }
    }

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    for y in 0..height {
        for x in 0..width {
            let y_mid = height / 2;
            let x_mid = width / 2;
            if y_mid != y && x_mid != x {
                if let Some(current) = map.get(&(x, y)) {
                    print!("{current}");
                    match (y < y_mid, x < x_mid) {
                        (true, true) => tl += *current,
                        (true, false) => tr += *current,
                        (false, true) => bl += *current,
                        (false, false) => br += *current,
                    }
                } else {
                    print!(".");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
    
    let total = tl * tr * bl * br;
    println!("Total {total} ({tl} {tr} {bl} {br})");
}
