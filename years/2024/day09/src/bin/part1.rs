use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;
use std::str::FromStr;

struct File<'a> {
    id: usize,
    blocks: Vec<&'a Block>,
}

impl<'a> File<'a> {
    fn new(id: usize, blocks: Vec<&'a Block>) -> Self {
        Self { id, blocks }
    }

    fn len(&self) -> usize {
        self.blocks.len()
    }

    fn add(&mut self, block: &'a Block) {
        self.blocks.push(block)
    }
}

enum Block {
    FREE,
    FULL(usize), // file id
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut files: HashMap<usize, File> = HashMap::new();
    let mut blocks: Vec<Block> = input
        .chars()
        .filter(|x| *x != '\n')
        .enumerate()
        .flat_map(|(i, x)| {
            match i % 2 {
                0 => {
                    // its a file
                    let size: usize = x.to_digit(10).unwrap() as usize;
                    let file_id = i / 2;
                    let file = File::new(file_id, vec![]);
                    files.insert(file_id, file);
                    (0..size)
                        .map(|x| {
                            let block = Block::FULL(file_id);
                            // file.add(&block);
                            block
                        })
                        .collect::<Vec<_>>()
                }
                _ => {
                    // it's a free section
                    let size: usize = x.to_digit(10).unwrap() as usize;
                    (0..size).map(|x| Block::FREE).collect()
                }
            }
        })
        .collect();

    for block in &blocks {
        if let Block::FULL(file_id) = block {
            files.get_mut(&file_id).unwrap().add(block);
        }
    }

    let mut last_full_block_pos = blocks.len()
        - 1
        - blocks
            .iter()
            .rev()
            .position(|x| {
                if let Block::FREE = x {
                    return false;
                } else {
                    return true;
                }
            })
            .unwrap();

    loop {
        let first_empty_block_pos = blocks
            .iter()
            .position(|x| {
                if let Block::FREE = x {
                    return true;
                } else {
                    return false;
                }
            })
            .unwrap();

        if first_empty_block_pos > last_full_block_pos {
            break;
        }

        match blocks.get(first_empty_block_pos).unwrap() {
            Block::FULL(_) => continue,
            Block::FREE => {
                // check to not move the last block multiple times
                // swap with the last
                blocks.swap(first_empty_block_pos, last_full_block_pos);
                last_full_block_pos = blocks.len()
                    - 1
                    - blocks
                        .iter()
                        .rev()
                        .position(|x| {
                            if let Block::FREE = x {
                                return false;
                            } else {
                                return true;
                            }
                        })
                        .unwrap();
            }
        }
    }

    for block in &blocks {
        match block {
            Block::FREE => eprint!("."),
            Block::FULL(id) => eprint!("{}", id),
        }
    }
    eprintln!("");

    let checksum: usize = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, x)| match x {
            Block::FREE => None,
            Block::FULL(id) => Some(id * i),
        })
        .sum();
    println!("{}", checksum);
}
