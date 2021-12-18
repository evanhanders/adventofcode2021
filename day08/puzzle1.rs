use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DAYS : usize = 80;

pub struct ParsedLine {
    line : String,
    digits : [String; 10],
    code : [String; 4],
    unique_lens : [u8; 4]
}

impl ParsedLine {
    fn new(line : String) -> ParsedLine {
        let mut digits : [String; 10] = Default::default();
        let mut code : [String; 4] = Default::default();
        let unique_lens : [u8; 4] = [2, 3, 4, 7];
        let linesplit = line.split(" | ").collect::<Vec<&str>>();
        for i in 0..linesplit.len() {
            if i == 0 {
                for (j, val) in linesplit[i].split(" ").collect::<Vec<&str>>().iter().enumerate() {
                    digits[j] = val.to_owned().to_owned();
                }
            } else if i == 1 {
                for (j, val) in linesplit[i].split(" ").collect::<Vec<&str>>().iter().enumerate() {
                    code[j] = val.to_owned().to_owned();
                }
            }
        }
        ParsedLine { line : line, digits : digits, code : code, unique_lens : unique_lens }
    }

    fn pline(&mut self) {
        println!("{}", self.line);
    }

    fn count_unique(&mut self) -> u8 {
        let mut counter : u8 = 0;
        for (i, val) in self.code.iter().enumerate() {
            if self.unique_lens.contains(&(val.len() as u8)) {
                counter += 1;
                println!("{}", val);
            }
        }
        return counter;
    }

}


fn main() {
    let mut raw_lines : Vec<ParsedLine> = Vec::new();
    let mut counter : u32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut thisline = ParsedLine::new(ip);
                counter += thisline.count_unique() as u32;
                raw_lines.push(thisline);
            }
        }
    }
    println!("there are {} unique vals", counter);
//
//    for count in input.split(",").collect::<Vec<&str>>().iter() {
//        let mut fish = LanternFish::new();
//        fish.counter = count.parse::<u8>().unwrap();
//        school.push(fish);
//    }
//
//    let mut newfish = 0;
//    for d in 0..DAYS {
//        println!("{}", d);
//        for i in 0..school.len() {
//            if school[i as usize].age() {
//                newfish += 1;
//            }
//        }
//        for i in 0..newfish {
//            school.push(LanternFish::new());
//        }
//        newfish = 0;
//        println!("{}", school.len());
//    }
//    for i in 0..school.len() {
//        print!("{},", school[i as usize].counter);
//    }
//    println!("");
//    println!("there are {} fish", school.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
