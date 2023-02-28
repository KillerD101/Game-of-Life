extern crate pancurses;

use pancurses::{initscr, Window, Input};

const DEFAULT_BOARD_WIDTH: usize = 50;
const DEFAULT_BOARD_HEIGHT: usize = 20;

struct Board {
    board: Vec<bool>,
    width: usize,
    height: usize
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let mut board = Board {
            board: Vec::new(),
            width: width,
            height: height
        };

        board.board.reserve(width * height);
        for _ in 0..(width * height) {
            board.board.push(false);
        }

        return board;
    }

    fn print(&self, window: &Window) {
        window.clear();

        for i in 0..self.height {
            for j in 0..self.width {
                window.printw(
                    if self.board[i * self.width + j] {"#"} else {"O"}
                );
            }
            window.printw("\n");
        }

        window.refresh();
    }

    fn set_state(&mut self, next: Vec<bool>) {
        self.board = next;
    }

    fn get(&self, col: usize, row: usize) -> bool {
        return self.board[col * self.width + row];
    }

    fn set(&mut self, col: usize, row: usize) {
        let index = col * self.width + row;

        self.board[index] = !self.board[index];
    }
}

fn get_neighbors(board: &Board, col: usize, row: usize) -> Vec<bool> {
    let mut neighbors: Vec<bool> = Vec::new();

    for c in -1..2 {
        for r in -1..2 {
            if c != 0 || r != 0 {
                let temp = board.board.get((
                    ((col as i32) + c) * (board.width as i32) + ((row as i32) + r)
                ) as usize); 
                if temp.is_some() == true {
                    neighbors.push(*temp.unwrap());
                }
            }
        }
    }

    return neighbors;
}

fn count(list: Vec<bool>) -> usize {
    let mut c: usize = 0;

    for item in list {
        if item { c += 1; }
    }

    return c;
}

fn print_break() {
    println!();

    for _ in 0..(DEFAULT_BOARD_WIDTH + 5) {
        print!("-");
    }

    println!();
    println!();
}

fn wait_for_input(window: &Window) -> Input{
    let mut i: Option<Input> = None;

    while i.is_none() {
        i = window.getch();
    }

    return i.unwrap();
}

fn main() {
    let mut board = Board::new(
        DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_HEIGHT
    );
    let window: Window = initscr();

    board.set(7, 7);
    board.set(7, 8);
    board.set(7, 6);
    board.set(6, 7);
    board.set(8, 7);
 
    board.print(&window);

    let mut running = true;

    if wait_for_input(&window) == Input::KeyBackspace {
        running = false;
    }

    while running {
        print_break();

        let mut next_state: Vec<bool> = Vec::new();

        for col in 0..board.height {
            for row in 0..board.width {
                let ncount = count(get_neighbors(&board, col, row));

                if board.get(col, row) {
                    next_state.push(ncount == 2 || ncount == 3);
                } else {
                    next_state.push(ncount == 3);
                }
            }
        }

        board.set_state(next_state);

        board.print(&window);

        if wait_for_input(&window) == Input::KeyBackspace {
            running = false;
        }
    }

    window.delwin();
}