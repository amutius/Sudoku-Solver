#![allow(non_snake_case)]
/*
 * IMPORTS
*/
use std::vec;
extern crate ansi_term;
use ansi_term::Style;
use ansi_term::Colour::RGB;
use std::process;
use std::io::{stdin,stdout,Write};
use std::fs;
use std::time::{Duration, Instant};
use std::thread;

struct Board {
    Fields: Vec<Vec<usize>>,
    Blocks: Vec<Vec<usize>>,
}

static BLENGTH: usize = 9;
static mut CNT: usize = 0;
/*
 * MAIN
*/
fn main() {
    println!("Sudoku Solver");
    println!("");
    // we our build board
    let mut board = Board{Fields:vec![vec![0;BLENGTH+1]; BLENGTH*BLENGTH],Blocks:vec![vec![0;BLENGTH];BLENGTH*BLENGTH]};
    // draw board
    draw(&board, true);
    // do input
    input(&mut board);
    // solve
    let start = Instant::now();
    let test = solve(&mut board);
    let duration = start.elapsed();

    println!("Time needed for soluton: {:?}", duration);
    unsafe {
        println!("{}", CNT);
    }
    for t in test {
        draw(&t, true);
    }
}

/*
 * INPUT MANAGEMENT
*/
fn input(board: &mut Board) {
    let mut input = String::new();
    println!("");
    println!("Input mode: 0 = manual input numbers / 1 = input from sudoku.csv");
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut input);
    if let Some('\n')=input.chars().next_back() {
       input.pop();
    }
    if input == "1" {
        let fcontent = fs::read_to_string("sudoku.csv").unwrap();
        let rows = fcontent.split("\n");
        for (i, row) in rows.into_iter().enumerate() {
            let val = row.split(",");
            for (k,column) in val.into_iter().enumerate() {
                if column == "" {
                    continue;
                }
                let m = column.parse::<usize>().is_ok();
                if !m {
                    println!("malformed data");
                    process::exit(0);
                }
                let toi8 = column.parse::<usize>().unwrap();
                board.Fields[i * BLENGTH + k][0] = toi8;
            }
        }
        draw(board, true);
    } else if input == "0" {
        println!("Place numbers on the board");
        println!("Format: X, Y, Number");
        println!("Exit: D(one)");
        // loop till end and get input
        while input != "D" {
            input = String::new();
            let _ = stdout().flush();
            let _ = stdin().read_line(&mut input);
            if let Some('\n')=input.chars().next_back() {
                input.pop();
            }
            let nums = input.split(",");
            let mut x = 0;
            let mut y = 0;
            let mut v = 0;
            for (i,n) in nums.into_iter().enumerate() {
                let m = n.parse::<usize>().is_ok();
                if !m {
                    println!("Wrong input");
                    continue;
                }
                let u = n.parse::<usize>().unwrap();
                if u > 9 {
                    println!("Wrong input");
                    continue;
                }
                if i == 0 {
                    x = u;
                } else if i == 1 {
                    y = u;
                } else if i == 2 {
                    v = n.parse::<usize>().unwrap();
                }
            }
            if x > 0 && y > 0 && v > 0 {
                board.Fields[(x-1) * BLENGTH + (y-1)][0] = v;
            }
            draw(&board, true);
        }
    } else {
        println!("Wrong input. Exiting..");
        process::exit(0);
    }
}
/*
 * DRAW
*/
fn draw(board: &Board, ln: bool) {
    let mut head = String::from("");
    let mut rownum = 0;
    for i in 0..board.Fields.len() {
        let num = board.Fields[i][0];
        if i == 0 {
            if ln {
                head += " ";
            }
            print!("{}", head);
            if ln {
                for number in 1..10 {
                    print!(" {}", Style::new().underline().paint(number.to_string()));
                }
            } else {
                println!(" _ _ _ _ _ _ _ _ _ ");
            }
        }
        if i == 0 || (i + 1 ) % 9 == 1 {
            rownum+=1;
            if ln {
                print!("\n");
                print!("{}", rownum);
                print!("|"); 
            }
        }
        let mut c = " ".to_string();
        if num != 0 {
            c = num.to_string();
        }
        if (i / BLENGTH + 1) % 3 == 0 {
            print!("{}", Style::new().underline().paint(c));
        } else {
            print!("{}", c);
        }
        if (i + 1) % 3 == 0 {
            print!("|"); 
        } else {
            print!("{}",RGB(100,100,100).paint("|"));
        }
    }
    println!();
}

/*
 * SOLVE
*/
// solve takes a mutable vec of vec of int8 
// and returns all vec of a vec of a vec of int8s
fn solve(board: &mut Board) -> Vec<Board> {
    unsafe {
        CNT+=1;
    }
    let mut boards: Vec<Board> = vec![];    
    let mut solved = true;
    let mut best = 0;
    let mut bestPos = 0;
    let mut found = 0;

    // iterate through every field -> total of 81 BLENGTH * BLENGTH
    for i in 0..board.Fields.len() {
        let mut amountFound = 0;
        let num = board.Fields[i][0];
        // num is set already
        if num != 0 {
            continue;
        }
        // we still have empty fields
        solved = false;
        
        // we try to calculate possible numbers for the field
        // linear calculations
        let rowStart = i - (i) % BLENGTH;
        let rowEnd = rowStart + BLENGTH;
        for rowI in rowStart..rowEnd {
            let rowNum = board.Fields[rowI][0];
            if rowNum != 0 && rowI != i {
                board.Fields[i][rowNum] = 1;
            }
        }
        // horizontal calculations
        let colStart = (i) % BLENGTH;
        for mut colI in 0..9 {
            colI = colStart + (colI * BLENGTH);
            let colNum = board.Fields[colI][0];
            if colNum != 0 && colI != i {
                board.Fields[i][colNum] = 1;
            }
        }
        // block calculations
        let x = colStart;
        let y = rowStart/BLENGTH;
        let blockModY = (y+1) % 3;
        let blockModX = (x+1) % 3;
        let mut blockStartX = x;
        let mut blockStartY = y;
        if blockStartX > 0 {
            if blockModX == 0 {
                blockStartX -= 2;
            } else if blockModX == 2 {
                blockStartX -= 1;
            }
        }
        if blockStartY > 0 {
            if blockModY == 0 {
                blockStartY -= 2;
            } else if blockModY == 2 {
                blockStartY -= 1;
            }
        }

        for blockXI in blockStartX..blockStartX+3 {
                for blockYI in blockStartY..blockStartY+3 {
                    let blockI = blockYI * BLENGTH + blockXI;
                    let blockField = board.Fields[blockI][0];
                    if blockField != 0 && blockI != i {
                    // also take out found cross possbilities alrdy
                        board.Fields[i][blockField] = 1;
                }
            }
        }
        for possible in 1..board.Fields[i].len() {
            if board.Fields[i][possible] != 1 {
                amountFound+=1;
                found = possible;
            }
        }
        // if nothing found, the board is broken
        if amountFound == 0 {
            return boards;
        }

        // fill if we got only one chance
        if amountFound == 1 {
            board.Fields[i][0] = found;
            return solve(board);
        }

        if best == 0 || amountFound < best {
            best = amountFound;
            bestPos = i;
        }

    }
    if solved {
        return vec![Board{Fields:board.Fields.clone(), Blocks:board.Blocks.clone()}];
    }
    // else we wanna guess
    for i in 1..board.Fields[bestPos].len() {
        if board.Fields[bestPos][i] != 1 {
            let mut testBoard = Board{Fields: board.Fields.clone(), Blocks:board.Blocks.clone()};
            testBoard.Fields[bestPos][0] = i;
            boards.append(&mut solve(&mut testBoard));
        }
    }
    return boards
}
