use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    const chars: usize = 12;
    let mut ox_strs : Vec<String> = Vec::new();
    let mut count: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let this_str = ip;
                ox_strs.push(this_str);

                count += 1;
            }
        }
        let mut co2_strs = ox_strs.clone();

        collapse_ox_vec(&mut ox_strs);
        collapse_co2_vec(&mut co2_strs);
        println!("{}, {}", ox_strs[0], ox_strs.len());
        println!("{}, {}", co2_strs[0], co2_strs.len());
        println!("ox bin {}, co2 bin {}", ox_strs[0], co2_strs[0]);
        let ox_int = isize::from_str_radix(&ox_strs[0][..], 2).unwrap();
        let co2_int = isize::from_str_radix(&co2_strs[0][..], 2).unwrap();
        println!("ox {}, co2 {}, power consumption {}", ox_int, co2_int, ox_int*co2_int)
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn collapse_ox_vec(v : &mut Vec<String>) {
        // find ox rate
        let mut charnum : usize = 0;
        let mut ox_char : char = 'A';
        while v.len() > 1 {
            let mut zeros: u32 = 0;
            println!("remain");
            for s in v.iter() {
                if '0' == s.chars().nth(charnum).unwrap() {
                    zeros += 1;
                }                
                println!("{}, {}, {}", charnum, s, zeros);
            }
            let count = v.len() as u32;
        

            let mut bad : Vec<usize> = Vec::new();
            let curr_len = v.len();
            if zeros > count/2 {
                ox_char = '0';
            } else {
                ox_char = '1';
            }
            for (pos, e) in v.iter().enumerate() {
                if e.chars().nth(charnum).unwrap() != ox_char {
                    bad.push(pos)
                }
            }
            for i in bad.iter().rev() {
                println!("{}, {}, {}", charnum, ox_char, v[*i]);
                v.remove(*i);
            }
            if charnum > 11 {
                break
            }
            charnum += 1;
        }
}

fn collapse_co2_vec(v : &mut Vec<String>) {
        // find co2 rate
        let mut charnum : usize = 0;
        let mut co2_char : char = 'A';
        while v.len() > 1 {
            let mut zeros: u32 = 0;
            println!("remain");
            for s in v.iter() {
                if '0' == s.chars().nth(charnum).unwrap() {
                    zeros += 1;
                }                
                println!("{}, {}, {}", charnum, s, zeros);
            }
            let count = v.len() as u32;
        

            let mut bad : Vec<usize> = Vec::new();
            let curr_len = v.len();
            if zeros > count/2 {
                co2_char = '1';
            } else {
                co2_char = '0';
            }
            for (pos, e) in v.iter().enumerate() {
                if e.chars().nth(charnum).unwrap() != co2_char {
//                    println!("{}, {}, {}", charnum, co2_char, e);
                    bad.push(pos)
                }
            }
            for i in bad.iter().rev() {
                v.remove(*i);
            }
            if charnum > 11 {
                break
            }
            charnum += 1;
        }
}
