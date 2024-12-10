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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Type {
    File,
    Space,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Block {
    file: Type,
    size: usize,
    id: i64,
}

fn main() {
    let rows = read();
    let mut blocks: Vec<Block> = vec![];
    let mut free = false;
    let mut current = 0;

    for c in rows[0].chars() {
        let size: usize = format!("{c}").parse().expect("Couldn't parse"); // Why no char parse???
        if free {
            blocks.push(Block {
                file: Type::Space,
                size: size,
                id: -1,
            });
        } else {
            blocks.push(Block {
                file: Type::File,
                size: size,
                id: current,
            });
            current += 1;
        }
        free = !free;
    }
    // Wrap the vec in spaces
    blocks.insert(
        0,
        Block {
            file: Type::Space,
            size: 0,
            id: -1,
        },
    );
    blocks.insert(
        blocks.len(),
        Block {
            file: Type::Space,
            size: 0,
            id: -1,
        },
    );

    let mut current_id = blocks[blocks.len() - 2].id;

    while current_id >= 0 {
        let mut current_index = 0;

        for i in 0..blocks.len() {
            if blocks[i].id == current_id {
                current_index = i;
                break;
            }
        }

        let current_block = blocks[current_index];
        for i in 0..current_index {
            let block = blocks[i];
            if block.file == Type::Space && block.size >= current_block.size {
                // First remove the old block and turn its two spaces into one space
                // Set the current index to be the size of the two spaces around it
                blocks[current_index] = Block {
                    file: Type::Space,
                    size: blocks[current_index - 1].size
                        + blocks[current_index + 1].size
                        + current_block.size,
                    id: -1,
                };
                // Remove the surrounding blocks
                blocks.remove(current_index + 1);
                blocks.remove(current_index - 1);

                // Next insert the new block with spacing around it
                // The front block always becomes 0 sized
                blocks[i] = Block {
                    file: Type::Space,
                    size: 0,
                    id: -1,
                };
                // The next block is the actual block
                blocks.insert(i + 1, current_block);
                // Any leftover space is included (Even 0)
                blocks.insert(
                    i + 2,
                    Block {
                        file: Type::Space,
                        size: block.size - current_block.size,
                        id: -1,
                    },
                );
                break;
            }
        }

        current_id -= 1;
    }

    let mut total: i64 = 0;

    let mut global_index = 0;
    for block in blocks {
        for _ in 0..block.size {
            if block.file == Type::File {
                total += global_index * block.id;
            }
            global_index += 1;
        }
    }
    println!("Total {total}");
}

fn print_blocks(blocks: &[Block]) {
    for block in blocks {
        for _ in 0..block.size {
            if block.file == Type::File {
                print!("{}", block.id);
            }
            if block.file == Type::Space {
                print!(".");
            }
        }
    }
    println!();
}
