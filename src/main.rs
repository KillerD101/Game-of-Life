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

    fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", if self.board[i * self.width + j] {'#'} else {'O'});
            }
            println!();
        }
    }
}

fn main() {
    let board = Board::new(
        DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_HEIGHT
    );
 
    board.print();
}