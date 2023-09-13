extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{
    keyboard::Key, Button, ButtonEvent, ButtonState, RenderArgs, RenderEvent, UpdateEvent,
};
use piston::window::WindowSettings;
use rand::{thread_rng, Rng};
use std::collections::LinkedList;
use std::iter::FromIterator;

const FPS: u64 = 5;
const BLOCKSIZE: i32 = 20;
const SCREEN_W: i32 = 40;
const SCREEN_H: i32 = 30;

const COLOR_BACKGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const COLOR_SNAKE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const COLOR_FOOD: [f32; 4] = [0.4, 1.0, 0.4, 1.0];

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Food {
    x: i32,
    y: i32,
}

struct Snake {
    body: LinkedList<(i32, i32)>,
}

impl Snake {
    fn travel(&mut self, dir: Direction) {
        let mut head = (*self.body.front().expect("Head is missing")).clone();
        match dir {
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Up => head.1 -= 1,
            Direction::Down => head.1 += 1,
        }

        self.body.push_front(head);
    }

    fn eat(&mut self, food: Food) -> bool {
        let head: (i32, i32) = *self.body.front().expect("Head is missing");
        let food_remains: bool = if head.0 == food.x && head.1 == food.y {
            false
        } else {
            self.body.pop_back();
            true
        };
        return food_remains;
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square((x * BLOCKSIZE) as f64, (y * BLOCKSIZE) as f64, 20_f64)
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(COLOR_SNAKE, square, transform, gl));
        });
    }
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    dir: Direction,
    food: Food,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(COLOR_BACKGROUND, gl);
        });

        // draw food
        let fx = self.food.x;
        let fy = self.food.y;
        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(
                COLOR_FOOD,
                graphics::rectangle::square(
                    (fx * BLOCKSIZE) as f64,
                    (fy * BLOCKSIZE) as f64,
                    20_f64,
                ),
                transform,
                gl,
            )
        });

        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        self.snake.travel(self.dir);
        let food = self.food;
        let ate: bool = !self.snake.eat(food);
        if ate {
            let mut rng = thread_rng();
            self.food.x = rng.gen_range(1..SCREEN_W - 1);
            self.food.y = rng.gen_range(1..SCREEN_H - 1);
        }
    }

    fn pressed(&mut self, btn: &Button) {
        let prev_dir = self.dir.clone();

        self.dir = match btn {
            &Button::Keyboard(Key::Up) if prev_dir != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if prev_dir != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if prev_dir != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if prev_dir != Direction::Left => Direction::Right,
            _ => prev_dir,
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Rust",
        [(SCREEN_W * BLOCKSIZE) as f64, (SCREEN_H * BLOCKSIZE) as f64],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut rng = thread_rng();
    let food_x: i32 = rng.gen_range(1..SCREEN_W - 1);
    let food_y: i32 = rng.gen_range(1..SCREEN_H - 1);

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(1, SCREEN_H / 2), (0, SCREEN_H / 2)]).into_iter()),
        },
        dir: Direction::Right,
        food: Food {
            x: food_x,
            y: food_y,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(FPS);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_) = e.update_args() {
            game.update();
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                game.pressed(&args.button);
            }
        }
    }
}
