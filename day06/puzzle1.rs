use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DAYS : usize = 80;

#[derive(Copy, Clone)]
pub struct LanternFish {
    counter : u8
}

impl LanternFish {
    fn new() -> LanternFish {
        let mut counter : u8 = 8;
        LanternFish { counter : counter }
    }

    fn age(&mut self) -> bool {
        if self.counter == 0 {
            self.counter = 6;
            return true;
        } else {
            self.counter -= 1;
            return false;
        }
    }
}


fn main() {
    let mut input : String = "3,4,3,1,2".to_string();
    let mut school : Vec<LanternFish> = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                input = ip;
            }
            break;
        }
    }
    println!("{}", input);

    for count in input.split(",").collect::<Vec<&str>>().iter() {
        let mut fish = LanternFish::new();
        fish.counter = count.parse::<u8>().unwrap();
        school.push(fish);
    }

    let mut newfish = 0;
    for d in 0..DAYS {
        println!("{}", d);
        for i in 0..school.len() {
            if school[i as usize].age() {
                newfish += 1;
            }
        }
        for i in 0..newfish {
            school.push(LanternFish::new());
        }
        newfish = 0;
        println!("{}", school.len());
    }
    for i in 0..school.len() {
        print!("{},", school[i as usize].counter);
    }
    println!("");
    println!("there are {} fish", school.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
