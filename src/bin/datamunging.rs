use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

struct Field {
    start_col: usize,
    end_col: usize
}

impl Field {
    fn extract_num(&self, line: &str) -> f64 {
        line[self.start_col..self.end_col].trim().replace("*", "").parse().unwrap()
    }

    fn extract_str<'a>(&'a self, line: &'a str) -> &'a str {
        line[self.start_col..self.end_col].trim()
    }
}

fn main() {   
    let max_temp = Field { start_col: 4, end_col:10};
    let min_temp = Field { start_col: 10, end_col: 14};
    let day = Field { start_col: 0, end_col: 4};

    if let Some((spread, name)) = compare_columns("weather.dat", max_temp, min_temp, day) {
        println!("Smallest spread was {} on day {}", spread, name);
    }

    let goals_for = Field { start_col: 44, end_col: 46};
    let goals_against = Field { start_col: 51, end_col: 53};
    let team_name = Field { start_col: 7, end_col: 23};

    if let Some((spread, name)) = compare_columns("football.dat", goals_for, goals_against, team_name) {
        println!("Smallest spread was {} for team {}", spread, name);
    }
}

/*
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
*/

fn compare_columns(filename: &str,
                   a_col: Field,
                   b_col: Field,
                   id_col: Field) -> Option<(f64, String)> {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    
    let mut seen_headers = false;
    let mut smallest_spread = None;
    let mut chosen_line = None;
    
    for line in file.lines() {
        // The first line contains headers, so skip it
        if !seen_headers {
            seen_headers = true;
            continue;
        }

        // Find the difference between the two columns
        let l = line.unwrap();
        let spread = (a_col.extract_num(&l) -  b_col.extract_num(&l)).abs();

        // If this is the highest (or only) difference we've seen so far, save it off
        if (smallest_spread.is_some() && spread < smallest_spread.unwrap()) || smallest_spread.is_none() {
            smallest_spread = Some(spread);
            chosen_line = Some(l);
        }
    }

    if let Some(line) = chosen_line {
        let team = id_col.extract_str(&line);
        return Some((smallest_spread.unwrap(), team.to_string()))
    } else {
        return None
    }
}
