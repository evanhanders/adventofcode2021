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
                for i in 0..chars {
                    if Some('0') == characters.next(){
                        zeros[i] += 1;
                    }
                }

                count += 1;
            }
        }
    
        // find gamma (most common) and epsilon (least common) bits
        let mut gamma = String::new();
        let mut epsilon = String::new();
        for i in 0..chars {
            println!("{}, {}", zeros[i], count);
            if zeros[i] > count/2 {
                // zero is most common
                gamma.push_str("0");
                epsilon.push_str("1");
            } else {
                // one is most common
                gamma.push_str("1");
                epsilon.push_str("0");
            }
        }
        println!("gamma bin {}, epsilon bin {}", gamma, epsilon);
        let gamma_int = isize::from_str_radix(&gamma[..], 2).unwrap();
        let epsilon_int = isize::from_str_radix(&epsilon[..], 2).unwrap();
        println!("gamma {}, epsilon {}, power consumption {}", gamma_int, epsilon_int, gamma_int*epsilon_int)
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
