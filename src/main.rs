use std::convert::TryInto;
use std::time::{Duration, Instant};

// TODO check why linter says its wrong while it still works
extern crate rand;
// use rand::prelude::*;
use rand::{Rng, thread_rng};

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod tetris;
use tetris::Tetromino;
mod board;
use board::Board;
mod interface;
use interface::IO;


struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let mut rng = thread_rng();
    // Init SDL and interface
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("tetris", interface::screen_size().0 as u32, interface::screen_size().1 as u32).position_centered().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut io = IO::new(window.into_canvas().build().unwrap());
    io.canvas.set_draw_color(Color::BLACK);
    io.canvas.clear();
    io.canvas.present();
    // Init game logic
    let mut pos = Position{x: board::board_size().0, y: 0};
    let mut piece_type: u32 = rng.gen_range(1..=7);
    let mut piece = Tetromino::new(piece_type);
    let mut board = Board{..Default::default()};
    let mut timer = Instant::now();
    let mut is_blocked = board.collision_checker(pos.x, pos.y, &piece);

    'main: loop {
        is_blocked = board.collision_checker(pos.x, pos.y, &piece);
        'event: for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..}    => {break 'main},
                _                                                                       => {},
                Event::KeyDown{keycode: Some(Keycode::Down), ..} => {if !is_blocked.down {pos.y += 1;}},
                Event::KeyDown{keycode: Some(Keycode::Left), ..} => {if !is_blocked.left {pos.x -= 1;}},
                Event::KeyDown{keycode: Some(Keycode::Right), ..} => {if !is_blocked.right {pos.x += 1;}},
                Event::KeyDown{keycode: Some(Keycode::Z), ..} => {if !is_blocked.rotate {piece.rotate();}},
            }
            break 'event;
        }
        if timer.elapsed().as_millis() >= 500 {
            timer = Instant::now();
            if !is_blocked.down {pos.y += 1;}
            else {
                board.write_back_board(pos.x, pos.y, &piece);
                board.filled_line_cleaner();
                pos = Position{x: board::board_size().0, y: 0};
                piece = Tetromino::new(piece_type);
            }
        }
        board.write_board(pos.x, pos.y, &piece);
        io.draw_board(&board);
        io.draw_overlay();
        io.canvas.present();
        std::thread::sleep(Duration::from_millis(50));
    }
    println!("Game Over!! score: {}", board.score);
}
