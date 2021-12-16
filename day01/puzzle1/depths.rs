use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut prev = u32::MAX;
    let mut curr = u32::MAX;
    let mut count = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                prev = curr;
                curr = ip.parse::<u32>().unwrap();
                if curr > prev{
                    count += 1;
                }
                println!("{}, {}, {}", prev, curr, count);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
