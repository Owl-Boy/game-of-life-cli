const HEIGHT: usize = 5;
const WIDTH: usize = 5;

type State = bool;
// type Board = [[State; WIDTH]; HEIGHT];
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

fn no_of_alive_nbrs(board: &Board, x: usize, y:usize) -> i8{
    let x = x as i32;
    let y = y as i32;
    let mut sum = 0;
    for a in x-1..=x+1{
        for b in y-1..=y+1{
            if (a,b) != (x, y) && board[rem(x, HEIGHT)][rem(y, WIDTH)]
                {sum += 1;};
        };
    };
    sum
}

fn rem(numerator: i32, divisor: usize) -> usize{
    let val = numerator % divisor as i32;
    if val < 0
        {(val + (divisor as i32)) as usize}
    else
        {val as usize}
}

fn next_board(board: &Board, height: &usize, width: &usize) -> Board{
    // let mut new_board: Board = [[DEAD; WIDTH]; HEIGHT];
    let mut new_board: Board = vec![vec![DEAD; *width]; *height];
    for x in 0..HEIGHT{
        for y in 0..WIDTH{
            new_board[x][y] = next_state(board[x][y],
                                         no_of_alive_nbrs(board, x, y));
        };
    };
    new_board
}

fn main() {
    println!("Hello, world!");
    // let mut example_board_0 = glider();
    // let mut example_board_1 = Board::string_to_board(_GLIDER_GUN);
    // loop {
    //     print!("\x1B[2J\x1B[1;1H");    // clears the screen
    //     println!("{example_board_1}");

    //     let temp = example_board_1.next_board();
    //     if temp == example_board_1 { break;}
    //     example_board_1 = temp;

    //     thread::sleep(time::Duration::from_millis(200));
    // }
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

