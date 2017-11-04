use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {   
    weather();
    football();
    let (spread, name) = football2("weather.dat", 4, 10, 10, 14, 0, 4);
    println!("Smallest spread was {} on day {}", spread, name);
    let (spread, name) = football2("football.dat", 44, 46, 51, 53, 7, 23);
    println!("Smallest spread was {} for team {}", spread, name);
}

fn weather() {   
    let f = File::open("weather.dat").unwrap();
    let file = BufReader::new(&f);
    let mut i = 0;
    let mut smallest_spread = 1_000_000.0;
    let mut day_number: u32 = 0;
    for line in file.lines() {
        i += 1;
        if i > 1 {
            let l = line.unwrap();
            let temp1 : &f64 = &l[4..10].trim().replace("*", "").parse().unwrap();
            let temp2 : &f64 = &l[10..14].trim().replace("*", "").parse().unwrap();
            let spread = temp1 - temp2;
            if spread < smallest_spread {
                smallest_spread = spread;
                day_number = *(&l[0..4].trim().parse().unwrap());
            }
        }
    }
    println!("Smallest spread was {} on day {}", smallest_spread, day_number);
}

fn football() {   
    let f = File::open("football.dat").unwrap();
    let file = BufReader::new(&f);
    let mut i = 0;
    let mut smallest_spread = 1_000_000.0;
    let mut team_line : String = String::new();
    let mut l: String = String::new();
    for line in file.lines() {
        i += 1;
        if i > 1 {
            l = line.unwrap();
            let temp1 : &f64 = &l[44..46].trim().replace("*", "").parse().unwrap();
            let temp2 : &f64 = &l[51..53].trim().replace("*", "").parse().unwrap();
            let spread = temp1 - temp2;
            if spread < smallest_spread {
                smallest_spread = spread;
                team_line = l;
            }
        }
    }
    let team = &team_line[7..23].trim();
    println!("Smallest spread was {} on day {}", smallest_spread, team);
}

fn football2(filename: &str,
             a_col_start: usize,
             a_col_end: usize,
             b_col_start: usize,
             b_col_end: usize,
             name_col_start: usize,
             name_col_end: usize) -> (f64, String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let mut i = 0;
    let mut smallest_spread = 1_000_000.0;
    let mut chosen_line : String = String::new();
    let mut l: String = String::new();
    for line in file.lines() {
        i += 1;
        if i > 1 {
            l = line.unwrap();
            let a: &f64 = &l[a_col_start..a_col_end].trim().replace("*", "").parse().unwrap();
            let b: &f64 = &l[b_col_start..b_col_end].trim().replace("*", "").parse().unwrap();
            let spread = a - b;
            if spread < smallest_spread {
                smallest_spread = spread;
                chosen_line = l;
            }
        }
    }
    let team = &chosen_line[name_col_start..name_col_end].trim();
    return (smallest_spread, team.to_string())
}
