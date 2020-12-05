use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


fn main() -> io::Result<()> {
    let f = File::open("day5.txt")?;
    let mut f = BufReader::new(f);

    let mut seats = Vec::<i32>::new();
    
    
    for line in f.lines() {
        let line = line.unwrap();

        let row = &line[..7];
        let column = &line[7..];


        let mut start = 0;
        let mut end = 127;

        let mut seat_id = 8;

        for i in row.chars() {
            
            if i == 'F'{
                end -= (end-start)/2;
            } else {
                start += (end-start)/2;
            }
        }
        
        if &row[7..] == "F" {
            seat_id *= start;
        } else {

            seat_id *= end;
        }

        start = 0;
        end = 7;

        for i in column.chars() {
            
            if i == 'L'{
                end -= (end-start)/2;
            } else {
                start += (end-start)/2;
            }
        }
        
        if &column[2..] == "L" {
            seat_id += start;
        } else {
            seat_id += end;
        }

        seats.push(seat_id);
    }

    let max_value = seats.iter().max().unwrap();

    let min_value = seats.iter().min().unwrap();


    for i in 21..900{
        if !seats.contains(&i) {
        
            if !seats.contains(&(&i+1)) && seats.contains(&(&i+2))
            {
                println!("{}",&i+1);
            }
        }

    }


    Ok(())
}
