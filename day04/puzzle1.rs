use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
pub struct BingoBoard {
    numbers : [[u32; 5]; 5],
    marked : [[bool; 5]; 5]
}

impl BingoBoard {
    fn new() -> BingoBoard {
        let mut numbers : [[u32; 5]; 5] = [[0; 5]; 5];
        let mut marked : [[bool; 5]; 5] = [[false; 5]; 5];
        BingoBoard { numbers: numbers, marked: marked }
    }

    fn add_num(&mut self, row : usize, col : usize, num: u32) {
        self.numbers[row][col] = num;
    }

    fn mark(&mut self, row : usize, col : usize) {
        self.marked[row][col] = true;
    }

    fn check(&mut self) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    break
                } else if j == 4 {
                    return true;
                }
            }
        }
        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[j][i] {
                    break
                } else if j == 4 {
                    return true;
                }
            }
        }
        return false;

    }
}


fn main() {
    const gameboards : usize = 100;
    let mut boards : [BingoBoard; gameboards] = [BingoBoard::new(); gameboards];
    let mut lineno : u32 =  0;
    let mut boarditer : u32 = 25;
    let mut boardnum  : i32 = -1;
    let mut callnums : Vec<u32> = Vec::new();
    let mut data : Vec<&str> = Vec::new();
    let mut num_arrs  : Vec<[[u32; 5]; 5]> = Vec::new();
    let mut bool_arrs : Vec<[[bool; 5]; 5]> = Vec::new();

    for i in 0..gameboards {
        boards[i as usize] = BingoBoard::new()
    }
    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if lineno == 0 {
                    for this_num in ip.split(",").collect::<Vec<&str>>().iter() {
                        let call_int = this_num.parse::<u32>().unwrap();
                        callnums.push(call_int);
//                        println!("{}",this_num);
                    }
                } else {
                    if ip.split(" ").collect::<Vec<&str>>().len() > 1 {
                        if boarditer == 25 {
                            boarditer = 0;
                            boardnum += 1;
                        }
                        let mut result = str::replace(&ip, "  ", " ");
                        if result.chars().nth(0).unwrap() == ' ' {
                            result = (&result[1..result.len()]).to_string();
                        }
                        for this_num in result.split(" ").collect::<Vec<&str>>().iter() {
                            let row = boarditer / 5;
                            let col = boarditer % 5;
                            let num = this_num.parse::<u32>().unwrap();
                            boards[boardnum as usize].add_num(row as usize, col as usize, num);

                            boarditer += 1;
                        }

                    }
                }
                lineno += 1;


            }
        }
    
    }
    for board in boards {
        println!("board");
        println!("{}, {}, {}, {}, {}", board.numbers[0][0], board.numbers[0][1], board.numbers[0][2], board.numbers[0][3], board.numbers[0][4]);
        println!("{}, {}, {}, {}, {}", board.numbers[1][0], board.numbers[1][1], board.numbers[1][2], board.numbers[1][3], board.numbers[1][4]);
        println!("{}, {}, {}, {}, {}", board.numbers[2][0], board.numbers[2][1], board.numbers[2][2], board.numbers[2][3], board.numbers[2][4]);
        println!("{}, {}, {}, {}, {}", board.numbers[3][0], board.numbers[3][1], board.numbers[3][2], board.numbers[3][3], board.numbers[3][4]);
        println!("{}, {}, {}, {}, {}", board.numbers[4][0], board.numbers[4][1], board.numbers[4][2], board.numbers[4][3], board.numbers[4][4]);
    }
    let mut finished = false;
    for num in callnums.iter() {
        for n in 0..boardnum+1 {
            for i in 0..5 {
                for j in 0..5 {
                    if boards[n as usize].numbers[i][j] == *num {
                        boards[n as usize].mark(i, j);
//                        println!("{},{},{}", i, j, boards[n as usize].marked[i][j])
                    }

                }
            }
            let winner = boards[n as usize].check();
//            println!("{},{},{}", n, num, winner);
            if winner {
                let mut unmarked_sum : u32 = 0;
                for i in 0..5 {
                    for j in 0..5 {
                        if !boards[n as usize].marked[i][j] {
                            unmarked_sum += boards[n as usize].numbers[i][j];
                        }
                    }
                }
                unmarked_sum *= num;
                println!("puzzle answer {}", unmarked_sum);
                finished = true;
                break;
            }
        }
        if finished {
            break;
        }
//        let mut count = 0;
//       for board in boards {
//            count += 1;
//            println!("board, {}", count);
//            println!("{}, {}, {}, {}, {}", board.marked[0][0], board.marked[0][1], board.marked[0][2], board.marked[0][3], board.marked[0][4]);
//            println!("{}, {}, {}, {}, {}", board.marked[1][0], board.marked[1][1], board.marked[1][2], board.marked[1][3], board.marked[1][4]);
//            println!("{}, {}, {}, {}, {}", board.marked[2][0], board.marked[2][1], board.marked[2][2], board.marked[2][3], board.marked[2][4]);
//            println!("{}, {}, {}, {}, {}", board.marked[3][0], board.marked[3][1], board.marked[3][2], board.marked[3][3], board.marked[3][4]);
//            println!("{}, {}, {}, {}, {}", board.marked[4][0], board.marked[4][1], board.marked[4][2], board.marked[4][3], board.marked[4][4]);
//        }

    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
