extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

use glutin_window::{GlutinWindow, OpenGL};
use opengl_graphics::GlGraphics;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{
    Button, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;
use std::collections::LinkedList;
use std::iter::FromIterator;
use Button::Keyboard;

pub struct Game {
    gl: GlGraphics,
    rows: u32,
    cols: u32,
    snake: Snake,
    just_eaten: bool,
    square_width: u32,
    food: Food,
    score: u32,
}

impl Game {
    pub fn update(&mut self, _args: &UpdateArgs) -> bool {
        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            return false;
        }

        if self.just_eaten {
            self.score += 1;
            self.just_eaten = false;
        }

        self.just_eaten = self.food.update(&self.snake);

        if self.just_eaten {
            use rand::thread_rng;
            use rand::Rng;

            let mut r = thread_rng();

            loop {
                let new_x = r.gen_range(0, self.cols);
                let new_y = r.gen_range(0, self.rows);
                if !self.snake.is_collide(new_x, new_y) {
                    self.food = Food { x: new_x, y: new_y };
                    break;
                }
            }
        }

        true
    }
}

impl Game {
    pub fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.d.clone();
        self.snake.d = match btn {
            Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        };
    }
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        // use graphics;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(args);
        self.food.render(&mut self.gl, args, self.square_width);
    }
}

pub struct Snake {
    gl: GlGraphics,
    snake_parts: LinkedList<SnakePiece>,
    width: u32,
    d: Direction,
}

impl Snake {
    pub fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
        let mut new_front: SnakePiece =
            (*self.snake_parts.front().expect("No front of snake found.")).clone();

        if (self.d == Direction::Up && new_front.1 == 0)
            || (self.d == Direction::Left && new_front.0 == 0)
            || (self.d == Direction::Down && new_front.1 == rows - 1)
            || (self.d == Direction::Right && new_front.0 == cols - 1)
        {
            return false;
        }

        match self.d {
            Direction::Up => new_front.1 -= 1,
            Direction::Down => new_front.1 += 1,
            Direction::Left => new_front.0 -= 1,
            Direction::Right => new_front.0 += 1,
        }

        if !just_eaten {
            self.snake_parts.pop_back();
        }

        if self.is_collide(new_front.0, new_front.1) {
            return false;
        }

        self.snake_parts.push_front(new_front);

        true
    }
    fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|p| x == p.0 && y == p.1)
    }
}

impl Snake {
    pub fn render(&mut self, args: &RenderArgs) {
        // use ::{graphics};

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .snake_parts
            .iter()
            .map(|p| SnakePiece(p.0 * self.width, p.1 * self.width))
            .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
            .collect();

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        })
    }
}

#[derive(Clone)]
pub struct SnakePiece(u32, u32);

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {
    pub fn update(&mut self, s: &Snake) -> bool {
        let front = s.snake_parts.front().unwrap();
        front.0 == self.x && front.1 == self.y
    }
}

impl Food {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        // use graphics;

        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl)
        })
    }
}

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
