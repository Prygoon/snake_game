use crate::direction::Direction;
use crate::food::Food;
use crate::snake::Snake;
use opengl_graphics::GlGraphics;
use piston::input::{Button, Button::Keyboard, Key, RenderArgs, UpdateArgs};

pub struct Game {
    pub gl: GlGraphics,
    pub rows: u32,
    pub cols: u32,
    pub snake: Snake,
    pub just_eaten: bool,
    pub square_width: u32,
    pub food: Food,
    pub score: u32,
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

    pub(crate) fn render(&mut self, args: &RenderArgs) {
        // use graphics;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(args);
        self.food.render(&mut self.gl, args, self.square_width);
    }
}
