use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut prev_sum = u32::MAX;
    let mut curr_sum = u32::MAX;
    let mut lineno = 0;
    let mut count = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                lineno += 1;
                a = b;
                b = c;
                c = ip.parse::<u32>().unwrap();
                if lineno < 3 {
                    continue;
                }
                prev_sum = curr_sum;
                curr_sum = a + b + c;
                if curr_sum > prev_sum{
                    count += 1;
                }
                println!("{}, {}, {}", prev_sum, curr_sum, count);
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
