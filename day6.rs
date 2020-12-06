use std::io::{self, BufReader};
use std::collections::{HashMap};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("day6.txt")?;
    let mut f = BufReader::new(f);
    
    let mut question_data = String::new();
    f.read_to_string(&mut question_data)?;


    let question_data: Vec<&str> = question_data.split("\n\n").collect();
    let mut total = 0;
    for i in question_data {

        let i: Vec<&str> = i.split_whitespace().collect();

        let mut answers = Vec::<char>::new();
        for k in &i {
            for x in k.chars() {
                answers.push(x);
            }
        }

        let people_count = &i.len();
        let mut map = HashMap::new();
        answers.sort();
        for i in &answers {
            let counter = map.entry(i).or_insert(0);
            *counter += 1;
        }

        for (_, val) in map {
            if &val ==  people_count {
            total += 1;
            }
        }
        
    }
    println!("{}",total);

    Ok(())
}
