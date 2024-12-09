use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
struct File {
    id: usize,
    start: usize,
    length: usize,
}

impl File {
    fn new(id: usize, start: usize, length: usize) -> Self {
        Self { id, start, length }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn add(&mut self) {
        self.length = self.length + 1;
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
                    (0..size)
                        .map(|_| {
                            let block = Block::FULL(file_id);
                            block
                        })
                        .collect::<Vec<_>>()
                }
                _ => {
                    // it's a free section
                    let size: usize = x.to_digit(10).unwrap() as usize;
                    (0..size).map(|_| Block::FREE).collect()
                }
            }
        })
        .collect();

    let mut free_space: Vec<(usize, usize)> = vec![];

    // create a mapping of file_ids to files
    let mut position = 0;
    let mut free_space_start: Option<usize> = None;
    for block in &blocks {
        match block {
            Block::FULL(file_id) => {
                match free_space_start {
                    Some(start) => {
                        free_space.push((start, position));
                        free_space_start = None
                    }
                    None => (),
                }
                match files.get_mut(&file_id) {
                    None => {
                        files.insert(*file_id, File::new(*file_id, position, 1));
                    }
                    Some(handle) => {
                        handle.add();
                    }
                }
            }
            Block::FREE => match free_space_start {
                None => free_space_start = Some(position),
                _ => (),
            },
        }

        position = position + 1
    }

    // algorithm could now be:
    // 1. check last_full_block_pos and get it's file id
    // 2. check for the File in the files HashMap to get the start and size
    // 3. iterate from the beginning and check if empty space can fit the file
    // 4. if it does -> move the entire file there
    // 5. repeat
    //
    dbg!(&files);

    let mut counter = 0;

    loop {
        // first find the last file on the disk
        if counter % 500 == 0 {
            eprintln!("{} files left to check", files.len());
        }


        let file_id = {
            let last_full_block_pos = blocks.len()
                - 1
                - match blocks.iter().rev().position(|x| match x {
                    Block::FREE => false,
                    Block::FULL(id) => files.contains_key(id),
                }) {
                    Some(pos) => pos,
                    None => break,
                };

            let last_full_block = &blocks[last_full_block_pos];
            match last_full_block {
                Block::FULL(file_id) => file_id,
                _ => panic!("something went wrong"),
            }
            .clone()
        };


        let file = match files.get(&file_id) {
            None => break, // no more files to move
            Some(file) => file,
        };

        // find free space to move the file to and move it if possible

        for i in 0..free_space.len() {
            let space = free_space.get(i).unwrap();
            if (space.1 - space.0) >= file.len() {
                // file can fit, move blocks

                let mut start = space.0;
                let mut end = space.1;
                for i in (0..file.len()) {
                    blocks.swap(space.0 + i, file.start + i);
                    start = start + 1;
                }
                // update free_space accordingly
                // if file fills empty space completely, remove free space
                if start == end {
                    free_space.remove(i);
                }

                // otherwise make it smaller
                if start < end {
                    *free_space.get_mut(i).unwrap() = (start, end);
                }

                break;
            }
        }

        files.remove(&file_id);
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
