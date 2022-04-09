use crate::direction::Direction;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::collections::LinkedList;

pub struct Snake {
    pub gl: GlGraphics,
    pub snake_parts: LinkedList<SnakePiece>,
    pub width: u32,
    pub d: Direction,
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
    pub fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|p| x == p.0 && y == p.1)
    }

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
pub struct SnakePiece(pub u32, pub u32);
