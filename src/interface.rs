use std::convert::TryInto;

use crate::tetris;
use crate::board;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub const fn block_size() -> usize {15}
pub const fn screen_size() -> (usize, usize) {(board::board_size().0*block_size()*2, board::board_size().1*block_size() )}
pub const fn default_color() -> Color {Color::BLACK}

pub fn color(n: u32) -> Result<Color,String> {
    match n {
        0 => Ok(Color::BLACK),
        1 => Ok(Color::RED),
        2 => Ok(Color::GREEN),
        3 => Ok(Color::BLUE),
        4 => Ok(Color::CYAN),
        5 => Ok(Color::MAGENTA),
        6 => Ok(Color::YELLOW),
        7 => Ok(Color::WHITE),
        8 => Ok(Color::GRAY),
        _ => Err("Unsupported color index".to_string())
    }
}

pub struct IO {
    // colors: [Color; color::COLOR_MAX.index() as usize],
    x_offset:       usize,
    y_offset:       usize,
    pub canvas:     sdl2::render::WindowCanvas,
    rects:          [[sdl2::rect::Rect; screen_size().0/block_size()]; screen_size().1/block_size()],
    bg_color:       Color
}

impl IO {
    pub fn new(window_canvas: sdl2::render::WindowCanvas) -> IO {
        IO {
            x_offset: (screen_size().0 - board::board_size().0 * block_size())/2,
            y_offset: (screen_size().1 - board::board_size().1 * block_size())/2,
            canvas: window_canvas,
            rects: IO::init_rects(),
            bg_color: default_color()
        }
    }
    // FIXME why am I substracting. rect suppose to be for all panels. just make the draw rect
    // based on the offset.
    fn init_rects() -> [[sdl2::rect::Rect; screen_size().0/block_size()]; screen_size().1/block_size()] {
        let mut rects = [[sdl2::rect::Rect::new(0, 0, 1, 1); screen_size().0/block_size()]; screen_size().1/block_size()];
        for j in 0..(screen_size().1/block_size()) {
            for i in 0..(screen_size().0/block_size()) {
                rects[j][i] = sdl2::rect::Rect::new(
                    (i*block_size()).try_into().unwrap(),
                    (j*block_size()).try_into().unwrap(),
                    block_size().try_into().unwrap(),
                    block_size().try_into().unwrap()
                )
            }
        }
        rects
    }
    pub fn draw_board(&mut self, game_board: &board::Board) {
        self.canvas.fill_rect(None).unwrap();
        for j in 0..board::board_size().1 {
            for i in 0..board::board_size().0 {
               self.canvas.set_draw_color(color(game_board.board[j][i]).unwrap());
               self.canvas.fill_rect(self.rects[j][i]).unwrap();
            }
        }
        self.canvas.set_draw_color(self.bg_color);
    }
}



