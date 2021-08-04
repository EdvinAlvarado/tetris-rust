use std::convert::TryInto;
use std::io::{Error, ErrorKind};
// mod tetris; // cant use mod if not it thinks it is under board directory just add mod in main
// file and use the following code line.
use crate::tetris;

// Handles adding usize and i32 and it will protect against overflow values.
pub fn pos_handler(u: usize, i: i32) -> Result<usize,i32> {
    let sum = u as i32 + i;
    if sum < 0 {Err(sum)} else {Ok(sum as usize)} // the number of Err does not matter
}

#[macro_export]
macro_rules! ph_u {
    ($u:expr, $i:expr) => {
        pos_handler($u,$i).unwrap()
    }
}

#[macro_export]
macro_rules! ph_uod {
    ($u:expr, $i:expr) => {
        pos_handler($u,$i).unwrap_or_default()
    }
}

pub const fn board_size() -> (usize, usize) {(20, 40)}

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
    fn pos_inbound(&self, x: i32, y: i32) -> Result<(), ErrorKind> {
        if (0..board_size().0 as i32).contains(&x) || (0..board_size().1 as i32).contains(&y) {
            return Ok(());
        }
        Err(ErrorKind::InvalidInput)
    }
    pub fn write_board(&mut self, x: i32, y: i32, piece: &tetris::Tetromino) {
        self.board = self.back_board;
        for j in 0..tetris::piece_size() {
            for i in 0..tetris::piece_size() {
                if piece.tetro[j][i] > 0 {
                    self.board[ph_uod!(j,y)][ph_uod!(i,x)] = piece.piece_type;
                }
            }
        }
    }
    pub fn write_back_board(&mut self, x: i32, y: i32, piece: &tetris::Tetromino) {
        for j in 0..tetris::piece_size() {
            for i in 0..tetris::piece_size() {
                if piece.tetro[j][i] > 0 {
                    self.back_board[ph_uod!(j,y)][ph_uod!(i,x)] = piece.piece_type;
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
    // FIXME fixed handling of directions
    #[allow(unused_must_use)]
    pub fn collision_checker(& self, x: i32, y: i32, piece: &tetris::Tetromino) -> Blocked {
        let mut pi =  *piece;
        pi.rotate();
        let mut dir = Blocked{..Default::default()};
        
        for j in 0..tetris::piece_size() {
            for i in 0..tetris::piece_size() {
                if piece.tetro[j][i] > 0 {
                    if pos_handler(i,x-1).is_err() {dir.left == true;}
                    else if self.back_board[ph_u!(j,y)][ph_u!(i,x-1)] > 0 {dir.left = true;}
                    
                    if ph_uod!(i,x+1) >= board_size().0 - 1 {dir.right == true;}
                    else if self.back_board[ph_u!(j,y)][ph_u!(i,x+1)] > 0 {dir.right = true;}
                    
                    if ph_u!(j,y) >= board_size().1 - 1 {dir.down == true;}
                    else if self.back_board[ph_u!(j,y+1)][ph_uod!(i,x)] > 0 {dir.down = true;}
                }
                if pi.tetro[j][i] > 0 {
                    // left || right || down || overlap
                    if pos_handler(i,x).is_err() || ph_uod!(i,x) >= board_size().0 || ph_uod!(j,y) >= board_size().1 || self.back_board[ph_uod!(j,y)][ph_uod!(i,x)] > 0 {dir.rotate = true;}
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

