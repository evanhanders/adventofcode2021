use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
pub struct Crab {
    position : i32
}

impl Crab {
    fn new(position : i32) -> Crab {
        Crab { position : position }
    }

    fn fuel_to(&mut self, endpoint : i32) -> i32 {
        let mut difference : i32 = endpoint - self.position;
        difference = i32::abs(difference);
        return difference;
    }
}


fn main() {
    let mut input : String = "16,1,2,0,4,2,7,1,2,14".to_string();
    let mut squad : Vec<Crab> = Vec::new();
    let mut maxpos : i32 = 0;
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
        let this_crab = Crab::new(count.parse::<i32>().unwrap());
        squad.push(this_crab);
        if this_crab.position > maxpos {
            maxpos = this_crab.position;
        }
    }

    let mut minfuel : i32 = maxpos*(squad.len() as i32);
    for p in 0..maxpos+1 {
        let mut thisfuel : i32 = 0;
        for i in 0..squad.len() {
            thisfuel += squad[i as usize].fuel_to(p);
        }
        if thisfuel < minfuel {
            minfuel = thisfuel;
        }
    }
    println!("{}", minfuel);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
