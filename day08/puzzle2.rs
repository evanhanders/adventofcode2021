use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

const CHARS : [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f',  'g'];
const ZERO : [bool; 7]  = [true, true, true, false, true, true, true];
const ONE : [bool; 7]   = [false, false, true, false, false, true, false];
const TWO: [bool; 7]    = [true, false, true, true, true, false, true];
const THREE: [bool; 7]  = [true, false, true, true, false, true, true];
const FOUR: [bool; 7]   = [false, true, true, true, false, true, false];
const FIVE: [bool; 7]   = [true, true, false, true, false, true, true];
const SIX: [bool; 7]    = [true, true, false, true, true, true, true];
const SEVEN: [bool; 7]  = [true, false, true, false, false, true, false];
const EIGHT: [bool; 7]  = [true; 7];
const NINE: [bool; 7]   = [true, true, true, true, false, true, true];
const MAPS : [[bool;7];10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

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
    possibilities : Vec<HashMap<char, bool>>,
    positions : [char; 7],
    unique_lens : [u8; 4],
    code : String
}

impl DigitCode {
    fn new(line : ParsedLine) -> DigitCode {
        let mut possibilities : Vec<HashMap<char, bool>> = Vec::new();
        let mut positions : [char; 7] = Default::default();
        let mut code : String = String::new();
        let unique_lens : [u8; 4] = [2, 3, 4, 7];

        for i in 0..7 {
            let mut changeable = HashMap::new();
            possibilities.push(changeable);
            for c in CHARS {
                possibilities[i].insert(c, true);
            }
        }
        DigitCode { line : line, positions : positions, unique_lens : unique_lens, code : code, possibilities : possibilities}
    }

    fn read(&mut self) {
        for numcode in self.line.digits.iter() {
            if numcode.len() == 2 {
                for c in CHARS {
                    if numcode.contains(c) {
                        for pos in [0, 1, 3, 4, 6] {
                           self.possibilities[pos].insert(c, false);
                        }
                    } else {
                        for pos in [2, 5] {
                           self.possibilities[pos].insert(c, false);
                        }
                    }
               }
           } else if numcode.len() == 4 {
                for c in CHARS {
                    if numcode.contains(c) {
                        for pos in [0, 4, 6] {
                           self.possibilities[pos].insert(c, false);
                        }
                    } else {
                        for pos in [1, 2, 3, 5] {
                           self.possibilities[pos].insert(c, false);
                        }
                    }
               }
           } else if numcode.len() == 3 {
                for c in CHARS {
                    if numcode.contains(c) {
                        for pos in [1, 3, 4, 6] {
                           self.possibilities[pos].insert(c, false);
                        }
                    } else {
                        for pos in [0, 2, 5] {
                           self.possibilities[pos].insert(c, false);
                        }
                    }
               }

               for pos in [1, 3, 4, 6] {
                   for c in numcode.chars() {
                       self.possibilities[pos].insert(c, false);
                   }
               }
           } 
        }

        // Trivially get position 0 from easy strings
        for (i, (c, b)) in self.possibilities[0].iter().enumerate() {
            if *b {
                self.positions[0] = *c;
            }
        }
        for i in 1..7 {
            self.possibilities[0].insert(self.positions[0], false);
        }

        //get position 2 by diffing 6-length strings compared to 1, 
        //then trivially get 5 from easy string
        for (i1, (c1,b1)) in self.possibilities[2].iter().enumerate() {
            for numcode in self.line.digits.iter() {
                if numcode.len() == 6 && !numcode.contains(*c1) && *b1 {
                    self.positions[2] = *c1;
                }
            }
        }
        self.possibilities[5].insert(self.positions[2], false);
        for (i, (c,b)) in self.possibilities[5].iter().enumerate() {
            if *b {
                self.positions[5] = *c;
            }
        }

        //get position 3 by diffing 6-length strings compared to 4, 
        //then trivially get 1 from easy string
        for (i1, (c1,b1)) in self.possibilities[3].iter().enumerate() {
            for numcode in self.line.digits.iter() {
                if numcode.len() == 6 && !numcode.contains(*c1) && *b1 {
                    self.positions[3] = *c1;
                }
            }
        }
        self.possibilities[1].insert(self.positions[3], false);
        for (i, (c,b)) in self.possibilities[1].iter().enumerate() {
            if *b {
                self.positions[1] = *c;
            }
        }
        //get position 4 -- only one five-length string has it.
        for (i, (c,b)) in self.possibilities[4].iter().enumerate() {
            let mut contained_in = 0;
            for numcode in self.line.digits.iter() {
                if numcode.len() == 6 && !numcode.contains(*c) && *b {
                    contained_in += 1;
                }
            }
            if contained_in == 1 {
                self.positions[4] = *c;
                break;
            }
        }
        self.possibilities[6].insert(self.positions[4], false);
        for (i, (c,b)) in self.possibilities[6].iter().enumerate() {
            if *b {
                self.positions[6] = *c;
            }
        }

        for digit in self.line.code.iter() {
            let mut thisline : [bool; 7] = [false; 7];
            let mut found : bool = false;
            for c1 in digit.chars() {
                for (i, c2) in self.positions.iter().enumerate() {
                    if c1 == *c2 {
                        thisline[i] = true;
                    }
                }
            }
            for (i, m) in MAPS.iter().enumerate() {
                for j in 0..7 {
                        if m[j] != thisline[j] {
                            break;
                        } else if j == 6 {
                            found = true;
                        }
                }
                if found {
                    self.code += &i.to_string();
                    break;
                }
            }
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
    let mut add_counter : u128 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                let mut thisline = ParsedLine::new(ip);
                let mut digit = DigitCode::new(thisline);
                digit.read();
                println!("{}", digit.code);
//                digit.print_eight();
                add_counter += digit.code.parse::<u128>().unwrap();
            }
        }
    }
    println!("add counter {}", add_counter);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
