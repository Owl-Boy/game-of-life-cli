mod lc;
mod start_configs;

use start_configs::*;
use lc::*;
use std::fmt;
use std::{thread, time};

type State = bool;
type Board = Vec<Vec<State>>;

const ALIVE: State = true;
const DEAD: State = false;

fn next_state(cell: State, no_of_alive_nbrs: i8) -> State{
    if cell {
        if no_of_alive_nbrs == 2 || no_of_alive_nbrs == 3
            {ALIVE}
        else
            {DEAD}
    } else {
        if no_of_alive_nbrs == 3
            {ALIVE}
         else
            {DEAD}
    }
}

fn next_board(board: &Board, width: &usize, height: &usize) -> Board{
    let mut new_board: Board = vec![vec![DEAD; *width]; *height];
    let mut any_ = false;
    for x in 0..*height{
        for y in 0..*width{
	    new_board[x][y] =  next_state(board[x][y],
                                          no_of_alive_nbrs(board, x, y,
							   width, height));
	}
    }
    new_board
}

fn no_of_alive_nbrs(board: &Board, x: usize, y:usize,
		    width: &usize, height: &usize) -> i8{
    let x = x as i32;
    let y = y as i32;
    lc!(1 ;
        a <- (x-1)..=(x+1), b <- (y-1)..=(y+1); 
        (a, b) != (x, y), board[rem(a, *height)][rem(b, *width)])
        .iter()
        .sum()
}


fn rem(numerator: i32, divisor: usize) -> usize{
    let val = numerator % divisor as i32;
    if val < 0
        {(val + (divisor as i32)) as usize}
    else
        {val as usize}
}

fn string_to_board(shape: &str, width: &usize, height: &usize) -> Board {
    let mut board: Board = vec![vec![DEAD; *width]; *height];
    let mut x = 0;
    let mut y = 0;
    for chr in shape.chars(){
	match chr {
            'O' => {
                board[x][y] = ALIVE;
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


fn print_board(board: &Board, width: &usize, height: &usize) {
    print!("╭─");
    for _ in 0..*width { print!("──"); }
    print!("─╮\n");
    
    for x in 0..*height {
        print!( "│ ");
	
        for y in 0..*width {
	    
            if board[x][y] {
                print!( "██");
	    }
	    else{
		print!( "  ");
            };
        }
	
        print!( " │\n");
    }
    
    print!( "╰─");
    for _ in 0..*width { print!( "──"); }
    println!( "─╯")
}

fn main() {
    println!("Hello, world!");
    let height = 50;
    let width = 50;
    // let mut example_board_0 = glider();
    let mut example_board_1 = string_to_board(_GLIDER_GUN, &width, &height);
    
    loop {
        print!("\x1B[2J\x1B[1;1H");    // clears the screen
	print_board(&example_board_1, &width, &height);

        let temp = next_board(&example_board_1, &width, &height);
	
        if temp == example_board_1 { break;}
        example_board_1 = temp;

        thread::sleep(time::Duration::from_millis(200));
    }
}
