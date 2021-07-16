use std::convert::TryInto;
// mod tetris; // cant use mod if not it thinks it is under board directory just add mod in main
// file and use the following code line.
use crate::tetris;

pub const fn board_size() -> (usize, usize) {(28, 56)}

pub struct Blocked {
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub rotate: bool,
}

// FIXME make back_board private while working with Default
pub struct Board {
    pub score: u32,
    pub board: [[u32; board_size().0]; board_size().1],
    pub back_board: [[u32; board_size().0]; board_size().1],
}

impl Board {
    fn check_line_filled(&self, line_index: usize) -> bool {
        let line = self.board.get(line_index).expect("line out of range");
        !line.iter().any(|x| x == &0)
    }
    fn pos_inbound(&self, x: usize, y: usize) -> bool {
        (0..board_size().0).contains(&x) || (0..board_size().1).contains(&y)
    }
    pub fn write_board(&mut self, x: usize, y: usize, piece: &tetris::Tetromino) {
        self.board = self.back_board;
        for j in piece.pos_delta[1]..tetris::piece_size() {
            for i in piece.pos_delta[0]..tetris::piece_size() {
                if piece.tetro[j][i] > 0 && self.pos_inbound(i+x, j+y) {
                    self.board[j+y][i+x] = piece.piece_type;
                }
            }
        }
    }
    pub fn write_back_board(&mut self, x: usize, y: usize, piece: &tetris::Tetromino) {
        for j in piece.v_iter() {
            for i in piece.h_iter() {
                if piece.tetro[j][i] > 0 && self.pos_inbound(i+x, j+y) {
                    self.back_board[j+y][i+x] = piece.piece_type;
                }
            }
        }
    }
    fn roll_lines(&mut self, line_to_delete: usize) {
        for line in line_to_delete..=0 {
            match line {
                0 => for n in self.back_board[line].iter_mut(){*n = 0},
                _ => self.back_board[line] = self.back_board[line-1],
            }
        } 
    }
    pub fn filled_line_cleaner(&mut self) {
        for line in 0..board_size().1 {
            if self.check_line_filled(line) {
                self.score += 1;
                self.roll_lines(line);
            }
        }
    }
    pub fn collision_checker(& self, x: usize, y: usize, piece: &tetris::Tetromino) -> Blocked {
        let mut pi =  *piece;
        let mut dir = Blocked{..Default::default()};
        for j in piece.v_iter() {
            for i in piece.h_iter() {
                if piece.tetro[j][i] > 0 {
                    dir.right   = x+i >= board_size().0 - 1 || self.back_board[y+j][x+i+1] > 0 || dir.right;
                    dir.down    = y+j >= board_size().1 - 1 || self.back_board[y+j+1][x+i] > 0 || dir.down; 
                } 
            }
        }
        pi.rotate();
        for j in pi.v_iter() {
            for i in pi.h_iter() {
                if pi.tetro[j][i] > 0 {
                    dir.rotate = x+i <= 0 || x+i >= board_size().0 - 1 || self.back_board[y+j][x+i] > 0 || dir.rotate;
                } 
            }
        }
        dir
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            score: 0,
            board: [[0; board_size().0]; board_size().1],
            back_board: [[0; board_size().0]; board_size().1],
        }
    }
}

impl Default for Blocked {
    fn default() -> Blocked {
        Blocked {
            left: false,
            right: false,
            down: false,
            rotate: false,
        }
    }
}

