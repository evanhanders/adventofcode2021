use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    const chars: usize = 12;
    let mut zeros: [u32; chars] = [0; chars];
    let mut count: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {

                let mut characters = ip.chars();
                for i in 0..chars-1 {
                    if Some('0') == characters.next(){
                        zeros[i] += 1;
                    }
                }

                count += 1;
            }
        }
    for i in 0..chars-1 {
        println!("{}, {}", zeros[i], count);
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
