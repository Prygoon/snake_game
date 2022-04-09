extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

use direction::Direction;
use food::Food;
use game::Game;
use glutin_window::{GlutinWindow, OpenGL};
use opengl_graphics::GlGraphics;
use piston::{
    event_loop::{EventLoop, EventSettings, Events},
    input::{ButtonEvent, ButtonState, RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use snake::{Snake, SnakePiece};
use std::collections::LinkedList;
use std::iter::FromIterator;

mod direction;
mod food;
mod game;
mod snake;

fn main() {
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 40;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    let width = COLS * SQUARE_WIDTH;
    let height = ROWS * SQUARE_WIDTH;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rows: ROWS,
        cols: COLS,
        square_width: SQUARE_WIDTH,
        just_eaten: false,
        food: Food { x: 1, y: 1 },
        score: 0,
        snake: Snake {
            gl: GlGraphics::new(opengl),
            snake_parts: LinkedList::from_iter((vec![SnakePiece(COLS / 2, ROWS / 2)]).into_iter()),
            width: SQUARE_WIDTH,
            d: Direction::Down,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(5);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }

    println!("Congratulations, your score was: {}", game.score);
}
