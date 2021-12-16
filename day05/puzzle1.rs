use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const GRIDSIZE : usize = 1000;

#[derive(Copy, Clone)]
pub struct VentGrid {
    grid : [[u8; GRIDSIZE]; GRIDSIZE],
}

impl VentGrid {
    fn new() -> VentGrid {
        let mut grid : [[u8; GRIDSIZE]; GRIDSIZE] = [[0; GRIDSIZE]; GRIDSIZE];
        VentGrid { grid : grid }
    }

    fn view_grid(&mut self) {
        println!("grid state:");
        for i in 0..GRIDSIZE {
            let mut row_string = String::new();
            for j in 0..GRIDSIZE {
                row_string.push_str(&format!("{:02}", self.grid[j][i]));
                row_string.push_str(", ")
            }
            println!("{}", row_string);
        }
    }

    fn add_line(&mut self, start : [usize; 2], end : [usize; 2]) {
        if start[0] == end[0] {
            if start[1] < end[1] {
                for j in start[1]..end[1]+1 {
                    self.grid[start[0]][j] += 1;
                }
            } else {
                for j in end[1]..start[1]+1 {
                    self.grid[start[0]][j] += 1;
                }
            }

        } else if start[1] == end[1] {
            if start[0] < end[0] {
                for i in start[0]..end[0]+1 {
                    self.grid[i][start[1]] += 1;
                }
            } else {
                for i in end[0]..start[0]+1 {
                    self.grid[i][start[1]] += 1;
                }
            }
        }
        return;
    }

    fn danger_count(&mut self) {
        let mut danger = 0;
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                if self.grid[j][i] >= 2 {
                    danger += 1
                }
            }
        }
        println!("danger spots {}", danger);

    }
}


fn main() {
    let mut my_grid : VentGrid = VentGrid::new();
    let mut startp : [usize; 2] = [0; 2];
    let mut endp : [usize; 2] = [0; 2];

    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                let mut pointind = 0;
                for coords in ip.split(" -> ").collect::<Vec<&str>>().iter() {
                    if pointind == 0 {
                        let xy = coords.split(",").collect::<Vec<&str>>();
                        startp[0] = xy[0].parse::<usize>().unwrap();
                        startp[1] = xy[1].parse::<usize>().unwrap();
                    } else {
                        let xy = coords.split(",").collect::<Vec<&str>>();
                        endp[0] = xy[0].parse::<usize>().unwrap();
                        endp[1] = xy[1].parse::<usize>().unwrap();
                    }
                    pointind += 1;
                }
            my_grid.add_line(startp, endp);
            }
        }
    
    }
    my_grid.view_grid();
    my_grid.danger_count();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
