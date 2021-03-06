mod lc;
mod start_configs;

use start_configs::*;
use lc::*;
use std::fmt;
use std::{thread, time};

const HEIGHT: usize = 33;
const WIDTH: usize = 71;

fn remainder(a: i32, b:usize) -> usize {
    let val = a % b as i32;
    let ans =  if val < 0 { b as i32 + val } else { val };
    ans as usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Dead,
    Alive,
}

const ALIVE: State = State::Alive;
const DEAD: State = State::Dead;

impl State {
   fn is_alive(self: &Self) -> bool {
       match self {
           &ALIVE => true,
           &DEAD => false       
       }
   } 

   fn next_state(self:Self, nbr_count: u32) -> State {
       let stay_alive = nbr_count == 2 || nbr_count == 3;
       let come_alive = nbr_count == 3;
       if self.is_alive() {
           if stay_alive { ALIVE } else { DEAD }
       } else {
           if come_alive { ALIVE } else { DEAD }
       }
   }
}

#[derive(Debug, PartialEq, Eq)]
struct Board {
    board: [[State; WIDTH]; HEIGHT]
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "╭─")?;
        for _ in 0..WIDTH { write!(f, "──")?; }
        write!(f, "─╮\n")?;

        for x in 0..HEIGHT {
            write!(f, "│ ")?;

            for y in 0..WIDTH {
                match self.board[x][y] {
                    ALIVE => write!(f, "██")?,
                    // ALIVE => write!(f, "■ ")?,
                    DEAD => write!(f, "  ")?
                };
            }

            write!(f, " │\n")?;
        }

        write!(f, "╰─")?;
        for _ in 0..WIDTH { write!(f, "──")?; }
        write!(f, "─╯")
    }
}

impl Board {
    fn new() -> Self {
        Board{ board: [[DEAD; WIDTH]; HEIGHT]}
    }

    fn count_alive_nbrs(self: &Self,x: usize, y: usize) -> u32 {
        let x = x as i32;
        let y = y as i32;
        lc!(1 ;
            a <- (x-1)..=(x+1), b <- (y-1)..=(y+1); 
            (a, b) != (x, y), self.board[remainder(a, HEIGHT)][remainder(b, WIDTH)].is_alive())
            .iter()
            .sum()
    }

    fn next_board(self: &Self) -> Self {
        let mut board = Board::new();

        for x in 0..HEIGHT {
            for y in 0.. WIDTH {
               board.board[x][y] = self.board[x][y].next_state(self.count_alive_nbrs(x, y));
            }
        }
        board
    }

    fn string_to_board(shape: &str) -> Self {
        let mut board = Board::new();
        let mut x = 0;
        let mut y = 0;
        for chr in shape.chars() {
            match chr {
                'O' => {
                    board.board[x][y] = ALIVE;
                    y += 1;
                },
                '.' => y += 1,
                '\n' => {
                    x +=1;
                    y = 0;
                }
                _ => {},
            };
        }
        board
    }
}

fn main() {
    // let mut example_board_0 = glider();
    let mut example_board_1 = Board::string_to_board(_GLIDER_GUN);
    loop {
        print!("\x1B[2J\x1B[1;1H");    // clears the screen
        println!("{example_board_1}");

        let temp = example_board_1.next_board();
        if temp == example_board_1 { break;}
        example_board_1 = temp;

        thread::sleep(time::Duration::from_millis(200));
    }
}

// fn glider() -> Board {
//     let mut board = Board::new();
//     board.board[0][1] = ALIVE;
//     board.board[1][2] = ALIVE;
//     board.board[2][0] = ALIVE;
//     board.board[2][1] = ALIVE;
//     board.board[2][2] = ALIVE;
//     board
// }

