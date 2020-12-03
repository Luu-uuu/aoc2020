use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let f = File::open("day1.txt")?;
    let f = BufReader::new(f);

    let mut known_values = HashSet::new();
    let mut difference_triplets = Vec::new();

    for line in f.lines() {
        //println!("{}", line.unwrap());
        let expense = line.unwrap();
        let expense: i32 = expense.parse().unwrap();
        let difference = 2020-expense;
        difference_triplets.push(difference);
    }

    known_values.clear();
    
       
    let f = File::open("day1.txt")?;
    let f = BufReader::new(f);
    for line in f.lines(){
        
        let expense = line.unwrap();
        let expense: i32 = expense.parse().unwrap();
        //let difference = 2020-expense;

        for i in difference_triplets.iter() {
          let difference = i-expense;
          if known_values.contains(&expense){
              println!("2020-ex{}",2020-i);
              println!("expense: {}",expense);
              //println!("diff: {}",difference);
              break;
          }else{
              known_values.insert(difference);
          }
        }
        
    }

    Ok(())
}
