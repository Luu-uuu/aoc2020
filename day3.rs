use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

struct Slope {
    x: usize,
    y: usize,
}


fn main() -> io::Result<()> {
    let f = File::open("day3.txt")?;
    let f = BufReader::new(f);
    
    let mut map: Vec::<String>  = Vec::new();

    for line in f.lines() {
        let horizontal_map_slice = line.unwrap();
        map.push(horizontal_map_slice);      
    }
    
    let slope = Slope {
        x: 1,
        y: 2,
    };

    let map_length = map[0].len();
    let map_height = map.len();

    let mut tree_count = 0;
    let mut k = 0;

    for i in (0 .. map_height-1).step_by(slope.y) {
        k = (k+slope.x) % (map_length);
        if map[i+slope.y].chars().nth(k).unwrap() == '#' {
            tree_count = tree_count + 1;
        }
    }

    println!("{}",tree_count);

    Ok(())
}
