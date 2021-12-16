use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DAYS : usize = 256;

#[derive(Copy, Clone)]
pub struct LanternFishBrood {
    counter : u8,
    fish : u128
}

impl LanternFishBrood {
    fn new(fish: u128) -> LanternFishBrood {
        let mut counter : u8 = 8;
        LanternFishBrood { counter : counter, fish : fish}
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
    let mut school : Vec<LanternFishBrood> = Vec::new();
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
        let this_count = count.parse::<u8>().unwrap();
        let mut brood_found : bool = false;
        for i in 0..school.len() {
            if school[i as usize].counter == this_count {
                school[i as usize].fish += 1;
                brood_found = true;
                break;
            }
        }
        if !brood_found {
            let mut brood = LanternFishBrood::new(1);
            brood.counter = this_count;
            school.push(brood);
        }
    }

    let mut newfish : u128 = 0;
    for d in 0..DAYS {
        println!("{}", d);
        for i in 0..school.len() {
            if school[i as usize].age() {
                newfish += school[i as usize].fish;
            }
        }
        school.push(LanternFishBrood::new(newfish));
        newfish = 0;
    }
    println!("");
    let mut numfish : u128 = 0;
    for brood in school {
        numfish += brood.fish;
    }
    println!("there are {} fish", numfish);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
