mod lc;

use lc::*;
use std::fmt;

const HEIGHT: usize = 5;
const WIDTH: usize = 5;

fn rem(a: i32, b:usize) -> usize {
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

#[derive(Debug)]
struct Board {
    board: [[State; WIDTH]; HEIGHT]
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..HEIGHT {
            for y in 0..WIDTH {
                match self.board[x][y] {
                    ALIVE => write!(f, "▣ ")?,
                    DEAD => write!(f, "□ ")?
                };
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Board {
    fn new() -> Self {
        Board{ board: [[DEAD; WIDTH]; HEIGHT]}
    }

    fn alive_nbrs(self: &Self,x: usize, y: usize) -> u32 {
        let x = x as i32;
        let y = y as i32;
        lc!(1 ;
            a <- (x-1)..=(x+1), b <- (y-1)..=(y+1); 
            (a, b) != (x, y), self.board[rem(a, HEIGHT)][rem(b, WIDTH)].is_alive())
            .iter()
            .sum()
    }

    fn new_board(self: &Self) -> Self {
        let mut board = Board::new();

        for x in 0..HEIGHT {
            for y in 0.. WIDTH {
               board.board[x][y] = self.board[x][y]
                   .next_state(self.alive_nbrs(x, y));
            }
        }
        board
    }
}

fn main() {
    let example_board_0 = Board {
        board: [
            [ DEAD,  DEAD,  DEAD,  DEAD, DEAD],
            [ DEAD,  DEAD, ALIVE,  DEAD, DEAD],
            [ DEAD, ALIVE, ALIVE, ALIVE, DEAD],
            [ DEAD,  DEAD, ALIVE,  DEAD, DEAD],
            [ DEAD,  DEAD,  DEAD,  DEAD, DEAD],
        ]
    };
    let example_board_1 = example_board_0.new_board();
    println!("{example_board_1}");
}

