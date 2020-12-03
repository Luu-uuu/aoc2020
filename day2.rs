use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("day2.txt")?;
    let f = BufReader::new(f);
    let mut valid_passwords = 0;

    for line in f.lines() {
        let password_input = line.unwrap();
        let password_input = password_input.split_whitespace();
        let password_input: Vec<&str> = password_input.collect();

        let limits: Vec<i32> = password_input[0].split("-").map(|s| s.parse::<i32>().unwrap()).collect();
        let lower_limit = limits[0] as usize;
        let upper_limit = limits[1] as usize;

        let character_req = &password_input[1][..1];

        let first_char = password_input[2].chars().nth(lower_limit-1);
        let second_char = password_input[2].chars().nth(upper_limit-1);

        //println!("{}", character_count);
        //if character_count >= lower_limit && character_count <= upper_limit {
        //    valid_passwords= valid_passwords + 1;
        let first_char = first_char.unwrap_or(' ');
        let second_char = second_char.unwrap_or(' ');

        if first_char != second_char {
            if first_char.to_string() == character_req || second_char.to_string() == character_req{
                valid_passwords = valid_passwords + 1;

            println!("{} {}",first_char, second_char);
            println!("requred {}", character_req);
            }
        }
        
    }
    println!("{}",valid_passwords);
    Ok(())
}
