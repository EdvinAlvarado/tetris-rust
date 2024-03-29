// std 
use std::convert::TryInto;
use std::time::{Duration, Instant};

// extern crates
extern crate rand;
use rand::{Rng, thread_rng};

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// project mod
mod tetris;
use tetris::Tetromino;
mod board;
use board::Board;
mod interface;
use interface::IO;

struct Position {x: i32, y: i32,}

fn main() {
    // Init randomizer
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
    let mut pos = Position{x: board::board_size().0 as i32 /2, y: 0}; // Assumes a 5x5 piece
    let mut piece = Tetromino::new(rng.gen_range(1..=7));
    let mut board = Board{..Default::default()};
    let mut timer = Instant::now();

    'main: loop {
        let mut is_blocked = board.collision_checker(pos.x, pos.y, &piece);
        'event: for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..}    => {break 'main},
                Event::KeyDown{keycode: Some(Keycode::Down), ..} => {if !is_blocked.down {pos.y += 1;}},
                Event::KeyDown{keycode: Some(Keycode::Left), ..} => {if !is_blocked.left {pos.x -= 1;}},
                Event::KeyDown{keycode: Some(Keycode::Right), ..} => {if !is_blocked.right {pos.x += 1;}},
                Event::KeyDown{keycode: Some(Keycode::Z), ..} => {if !is_blocked.rotate {piece.rotate();}},
                _ => {for ev in event_pump.poll_iter() {match ev {_ => {}}}},
            }
            is_blocked = board.collision_checker(pos.x, pos.y, &piece);
            break 'event;
        }
        if timer.elapsed().as_millis() >= 500 {
            timer = Instant::now();
            if !is_blocked.down {pos.y += 1;}
            else {
                board.write_back_board(pos.x, pos.y, &piece);
                board.filled_line_cleaner();
                pos = Position{x: board::board_size().0 as i32/2, y: 0};
                piece = Tetromino::new(rng.gen_range(1..=7));
            }
        }
        board.write_board(pos.x, pos.y, &piece);
        io.draw_board(&board);
        io.draw_overlay();
        io.canvas.present();
        // TODO Debug
        println!("{}\t{}", pos.x, pos.y);
        std::thread::sleep(Duration::from_millis(50));
    }
    println!("Game Over!! score: {}", board.score);
}
