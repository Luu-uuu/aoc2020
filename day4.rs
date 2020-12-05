use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

}


fn validate_passport(passport: &Passport) -> bool {
    if passport.byr.is_none() || passport.iyr.is_none() || passport.eyr.is_none() || passport.hgt.is_none() ||
       passport.hcl.is_none() || passport.ecl.is_none() || passport.pid.is_none() {
           return false;
       }

    let byr = passport.byr.as_ref().unwrap();

    if !(byr.parse::<i64>().unwrap() >= 1920 && byr.parse::<i64>().unwrap() <= 2002) {
        return false;
    }

    let iyr = passport.iyr.as_ref().unwrap();

    if !(iyr.parse::<i64>().unwrap() >= 2010 && iyr.parse::<i64>().unwrap() <= 2020) {
        return false;
    }

    let eyr = passport.eyr.as_ref().unwrap();

    if !(eyr.parse::<i64>().unwrap() >= 2020 && eyr.parse::<i64>().unwrap() <= 2030) {
        return false;
    }

    let hgt = passport.hgt.as_ref().unwrap();

    if hgt.contains("cm") {

        let hgt_num = &hgt[..hgt.len()-2];
        let hgt_num = hgt_num.parse::<i64>().unwrap();

        if !(hgt_num >= 150 && hgt_num <= 193) {
            return false;
        }

    } else if hgt.contains("in") {

        let hgt_num = &hgt[..hgt.len()-2];
        let hgt_num = hgt_num.parse::<i64>().unwrap();

        if !(hgt_num >= 59 && hgt_num <= 76) {
            return false;
        }

    } else {

        return false;
    }

    
    let hcl = passport.hcl.as_ref().unwrap();
    let hex = "0123456789abcdef";
    if &hcl[..1] != "#" || hcl.len() != 7  || hcl[1..].chars().any(|c| !hex.contains(c)){
            return false;
        
    }

    
    let ecl = passport.ecl.as_ref().unwrap();
    
    let valid_strings = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    if !valid_strings.contains(&ecl.as_str()) {
        return false;
    }

    
    let pid = passport.pid.as_ref().unwrap();
    
    if pid.len() != 9 {
        return false;
    }


    return true;
}

fn main() -> io::Result<()> {
    let f = File::open("day4.txt")?;
    let mut f = BufReader::new(f);
    
    let mut passports: Vec::<Passport>  = Vec::new();
    
    let mut passport_data = String::new();
    f.read_to_string(&mut passport_data)?;


    let passport_data: Vec<&str> = passport_data.split("\n\n").collect();
    for i in passport_data {

        let i: Vec<&str> = i.split_whitespace().collect();


        let mut passport_entry = Passport::new();
        for j in i {

            let j: Vec<&str> = j.split(":").collect();

            let passport_attr = j[0];
            match passport_attr {
                "byr" => passport_entry.byr = Some(j[1].to_string()),
                "iyr" => passport_entry.iyr = Some(j[1].to_string()),
                "eyr" => passport_entry.eyr = Some(j[1].to_string()),
                "hgt" => passport_entry.hgt = Some(j[1].to_string()),
                "hcl" => passport_entry.hcl = Some(j[1].to_string()),
                "ecl" => passport_entry.ecl = Some(j[1].to_string()),
                "pid" => passport_entry.pid = Some(j[1].to_string()),
                "cid" => passport_entry.cid = Some(j[1].to_string()),
                _ => println!("some error probably happened"),
            }

        }

        passports.push(passport_entry);
    }

    let mut valid = 0;
    for i in passports {
        if validate_passport(&i) {
            valid = valid + 1;
        }
    }

    println!("{}", valid);
    

    Ok(())
}
