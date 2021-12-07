use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut x: u32 = 0;
    let mut z: u32 = 0;
    x = 0;
    z = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let vec = ip.split(" ").collect::<Vec<&str>>();
                let num = vec[1].parse::<u32>().unwrap();
                if ip.contains("forward") {
                    x += num;
                } else if ip.contains("down") {
                    z += num;
                } else if ip.contains("up") {
                    z -= num;
                }
            println!("{}, {}", x, z);
            println!("{}", x*z)
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
