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

pub struct DigitCode {
    line : ParsedLine,
    digit_chars : [String; 10],
    positions : [char; 7],
    easy_strings : [String; 4],
    unique_lens : [u8; 4],
    code : String
}

impl DigitCode {
    fn new(line : ParsedLine) -> DigitCode {
        let mut positions : [char; 7] = Default::default();
        let mut easy_strings: [String; 4] = Default::default();
        let mut digit_chars: [String; 10] = Default::default();
        let mut code : String = String::new();
        let unique_lens : [u8; 4] = [2, 3, 4, 7];
        DigitCode { line : line, positions : positions, easy_strings : easy_strings, unique_lens : unique_lens, digit_chars : digit_chars, code : code }
    }

    fn read_easy(&mut self) {
        // 0 - 6 chars
        // 1 - 2 chars
        // 2 - 5 chars
        // 3 - 5 chars
        // 4 - 4 chars
        // 5 - 5 chars
        // 6 - 6 chars
        // 7 - 3 chars
        // 8 - 7 chars
        // 9 - 6 chars
        // positions
        // 0 - done
        // 1 - picked by 3
        // 2 - done
        // 3 - done  
        // 4 - only missing from one five-length.
        // 5 - picked by 2.
        // 6
        for numcode in self.line.digits.iter() {
           if numcode.len() == 2 {
               self.easy_strings[0] = numcode.to_owned();
           } else if numcode.len() == 4 {
               self.easy_strings[1] = numcode.to_owned();
           } else if numcode.len() == 3 {
               self.easy_strings[2] = numcode.to_owned();
           } else if numcode.len() == 7 {
               self.easy_strings[3] = numcode.to_owned();
           }
        }
        //get position 0
        for c in self.easy_strings[2].chars() {
            if !self.easy_strings[0].contains(c) {
                self.positions[0] = c;
                println!("{}", c);
            }
        }
        //get position 2
        for c in self.easy_strings[0].chars() {
            for numcode in self.line.digits.iter() {
                if numcode.len() == 6 && !numcode.contains(c) {
                    self.positions[2] = c;
                    println!("{}", c);
                }
            }
        }
        //trivially get position 5
        for c in self.easy_strings[0].chars() {
            if c != self.positions[2] {
                self.positions[5] = c;
                    println!("{}", c);
            }
        }
        //get position 3
        for c in self.easy_strings[1].chars() {
            for numcode in self.line.digits.iter() {
                if numcode.len() == 6 && !numcode.contains(c)  && numcode.contains(self.positions[2]) {
                    self.positions[3] = c;
                    println!("{}", c);
                }
            }
        }
        //trivially get position 1
        for c in self.easy_strings[1].chars() {
            if c != self.positions[2] && c != self.positions[3] && c != self.positions[5] {
                self.positions[1] = c;
                    println!("{}", c);
            }
        }
        //get position 4
        for c in self.easy_strings[3].chars() {
            for numcode in self.line.digits.iter() {
                if numcode.len() == 5 && !numcode.contains(c) {
                    if !self.positions.contains(&c) {
                        self.positions[4] = c;
                        println!("{}", c);
                    }
                }
            }
        }
        // final position
        let chars = ['a', 'b', 'c', 'd', 'e', 'f',  'g'];
        for c in chars.iter() {
            if !self.positions.contains(&c) {
                self.positions[6] = c.to_owned();
                        println!("{}", c);
            }
        }

        for (i, c) in self.positions.iter().enumerate() {
            if i == 0 {
                for j in [0, 2, 3, 5, 6, 7, 8, 9] {
                    self.digit_chars[j].push(*c);
                }
            }
            if i == 1 {
                for j in [0, 4, 5, 6, 8, 9] {
                    self.digit_chars[j].push(*c);
                }
            }
            if i == 2 {
                for j in [0, 1, 2, 3, 4, 7, 8, 9] {
                    self.digit_chars[j].push(*c);
                }
            }
            if i == 3 {
                for j in [2, 3, 4, 5, 6, 8, 9] {
                    self.digit_chars[j].push(*c);
                }
            }
            if i == 4 {
                for j in [0, 2, 6, 8] {
                    self.digit_chars[j].push(*c);
                }
            }
            if i == 5 {
                for j in [0, 1, 3, 4, 5, 6, 7, 8, 9] {
                    self.digit_chars[j].push(*c);
                }
            }
            if i == 6 {
                for j in [0, 2, 3, 5, 6, 8, 9] {
                    self.digit_chars[j].push(*c);
                }
            }
        }

        for s in self.digit_chars.iter() {
            println!("{}", s);
        }


        for codedigit in self.line.code.iter() {
            println!("need to convert to digits: code {}", codedigit);
        }
    }

    fn print_eight(&mut self) {
        println!(" {}{}{} ", self.positions[0], self.positions[0], self.positions[0]);
        println!("{}   {}",  self.positions[1], self.positions[2]);
        println!("{}   {}",  self.positions[1], self.positions[2]);
        println!(" {}{}{} ", self.positions[3], self.positions[3], self.positions[3]);
        println!("{}   {}",  self.positions[4], self.positions[5]);
        println!("{}   {}",  self.positions[4], self.positions[5]);
        println!(" {}{}{} ", self.positions[6], self.positions[6], self.positions[6]);
    }
}




fn main() {
    let mut raw_lines : Vec<ParsedLine> = Vec::new();
    let mut counter : u32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./fake_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                let mut thisline = ParsedLine::new(ip);
                let mut digit = DigitCode::new(thisline);
                digit.read_easy();
                digit.print_eight();
//                counter += thisline.count_unique() as u32;
//                raw_lines.push(thisline);
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
