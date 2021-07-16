use std::fmt;

pub const fn piece_size() -> usize {5}
fn tetromino_pieces(piece_type: u32) -> Result<[[u32; piece_size()]; piece_size()],String> {
    match piece_type {
        1 =>    Ok([
                [0,0,1,0,0],
                [0,0,1,0,0],
                [0,0,1,0,0],
                [0,0,1,0,0],
                [0,0,0,0,0]]),
        2 =>    Ok([
                [0,0,1,0,0],
                [0,0,1,0,0],
                [0,1,1,0,0],
                [0,0,0,0,0],
                [0,0,0,0,0]]),
        3 =>    Ok([
                [0,0,1,0,0],
                [0,0,1,0,0],
                [0,0,1,1,0],
                [0,0,0,0,0],
                [0,0,0,0,0]]),
        4 =>    Ok([
                [0,0,0,0,0],
                [0,1,1,0,0],
                [0,1,1,0,0],
                [0,0,0,0,0],
                [0,0,0,0,0]]),
        5 =>    Ok([
                [0,0,0,0,0],
                [0,0,1,0,0],
                [0,0,1,1,0],
                [0,0,0,1,0],
                [0,0,0,0,0]]),
        6 =>    Ok([
                [0,0,0,0,0],
                [0,0,1,0,0],
                [0,1,1,0,0],
                [0,1,0,0,0],
                [0,0,0,0,0]]),
        7 =>    Ok([
                [0,0,0,0,0],
                [0,0,1,0,0],
                [0,0,1,1,0],
                [0,0,0,1,0],
                [0,0,0,0,0]]),
        _ =>    Err("piece type not supported".to_string())
    }
}

fn tetromino_dimensions(piece_type: u32) -> Result<[usize; 4],String> {
    match piece_type {
        1 => Ok([2,0,2,1]),
        2 => Ok([1,0,2,2]),
        3 => Ok([2,0,1,2]),
        4 => Ok([1,1,2,2]),
        5 => Ok([2,1,1,1]),
        6 => Ok([1,1,2,1]),
        7 => Ok([2,1,1,1]),
        _ => Err("piece type is supported".to_string())
    }
}

#[derive(Copy, Clone)]
pub struct Tetromino {
    pub piece_type: u32,
    pub tetro:      [[u32; piece_size()]; piece_size()],
    pub pos_delta: [usize; 4],
}

impl Tetromino {
    pub fn new(piece_number: u32) -> Tetromino {
        Tetromino {
            piece_type: piece_number + 1,
            tetro: tetromino_pieces(piece_number).unwrap(),
            pos_delta: tetromino_dimensions(piece_number).unwrap(),
        }
    }
    pub fn rotate(&mut self) {
        let mut rotated_tetro: [[u32; piece_size()]; piece_size()] = [[0; piece_size()]; piece_size()];
        for y in 0..piece_size() {
            for x in 0..piece_size() {
                rotated_tetro[y][x] = self.tetro[piece_size()-1-x][y];
            }
        }
        let mut temp_arr = [0; 4];
        for i in 0..self.pos_delta.len() {
            match i {
                0 => temp_arr[0] = *self.pos_delta.iter().last().unwrap(),
                _ => temp_arr[i] = self.pos_delta[i-1]
            }
        }
        self.pos_delta  = temp_arr;
        self.tetro      = rotated_tetro;
    }
    pub fn h_iter(&self) -> impl Iterator<Item=usize> {
        self.pos_delta[0]..(piece_size()-self.pos_delta[2])
    }
    pub fn v_iter(&self) -> impl Iterator<Item=usize> {
        self.pos_delta[1]..(piece_size()-self.pos_delta[3])
    }
}

// permits printing
impl fmt::Debug for Tetromino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tetromino")
        .field("piece_type", &self.piece_type)
        .field("tetro", &self.tetro)
        .finish()
    }
} 
