use bracket_lib::prelude::*;

use crate::{
    board::Board,
    snake::{Direction, Point, Snake},
};

const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    GameOver,
}

pub struct SnakeGame {
    mode: GameMode,
    board: Board,
    snake: Snake,
    current_direction: Direction,
    current_length: u32,
    frame_time: f32,
}

impl SnakeGame {
    pub fn new() -> Self {
        SnakeGame {
            mode: GameMode::Menu,
            board: Board::new(80, 50),
            snake: Snake::new(vec![(40, 25), (41, 25), (42, 25)]),
            current_direction: Direction::Left,
            current_length: 3,
            frame_time: 0.0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "~~ S N A K E ~~");
        ctx.print_centered(8, "Press [P] to play");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                _ => {}
            }
        }
    }

    fn drop_apples(&mut self, count: u32) {
        let mut rng = RandomNumberGenerator::new();

        for _ in 0..count {
            let x = rng.range(1, self.board.width() - 1);
            let y = rng.range(1, self.board.height() - 1);

            self.board.add_apple(Point(x, y));
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;

        self.board = Board::new(80, 50);
        self.snake = Snake::new(vec![(40, 25), (41, 25), (42, 25)]);
        self.current_direction = Direction::Left;
        self.current_length = self.snake.segments().len() as u32;
        self.drop_apples(10);
        self.frame_time = 0.0;
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Snake is dead !!!");
        ctx.print_centered(8, "[P] Play again");
        ctx.print_centered(9, "[Q] Quit game");

        self.mode = GameMode::GameOver;

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn get_allowed_direction(&self, direction: Direction) -> Direction {
        match direction {
            Direction::Up if self.current_direction != Direction::Down => Direction::Up,
            Direction::Down if self.current_direction != Direction::Up => Direction::Down,
            Direction::Left if self.current_direction != Direction::Right => Direction::Left,
            Direction::Right if self.current_direction != Direction::Left => Direction::Right,
            _ => self.current_direction,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Playing;
        ctx.cls();

        self.render_board(ctx);
        self.render_snake(ctx);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => self.mode = GameMode::GameOver,
                VirtualKeyCode::Up  => self.current_direction = self.get_allowed_direction(Direction::Up),
                VirtualKeyCode::Down => self.current_direction = self.get_allowed_direction(Direction::Down),
                VirtualKeyCode::Left => self.current_direction =  self.get_allowed_direction(Direction::Left),
                VirtualKeyCode::Right => self.current_direction = self.get_allowed_direction(Direction::Right),
                _ => {}
            }
        }

        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
        } else {
            return;
        }

        self.snake.move_to(self.current_direction, &mut self.board);

        self.refill_apples();

        if !self.snake.is_alive() {
            self.dead(ctx);
        }
    }

    fn refill_apples(&mut self) {
        if self.current_length < self.snake.segments().len() as u32 {
            self.current_length = self.snake.segments().len() as u32;
            self.drop_apples(2);
        }
    }

    fn render_board(&self, ctx: &mut BTerm) {
        ctx.set(
            0,
            0,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('+'),
        );

        ctx.set(
            self.board.width() - 1,
            0,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('+'),
        );

        ctx.set(
            0,
            self.board.height() - 1,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('+'),
        );

        ctx.set(
            self.board.width() - 1,
            self.board.height() - 1,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('+'),
        );

        for x in 1..self.board.width()-1 {
            ctx.set(x, 0, RGB::named(WHITE), RGB::named(BLACK), to_cp437('-'));
            ctx.set(
                x,
                self.board.height() - 1,
                RGB::named(WHITE),
                RGB::named(BLACK),
                to_cp437('-'),
            );
        }

        for y in 1..self.board.height()-1 {
            ctx.set(0, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437('|'));
            ctx.set(
                self.board.width() - 1,
                y,
                RGB::named(WHITE),
                RGB::named(BLACK),
                to_cp437('|'),
            );
        }

        for apple in self.board.apples() {
            ctx.set(
                apple.x(),
                apple.y(),
                RGB::named(RED),
                RGB::named(BLACK),
                to_cp437('@'),
            );
        }
    }

    fn render_snake(&self, ctx: &mut BTerm) {
        for segment in &self.snake.segments {
            ctx.set(
                segment.x(),
                segment.y(),
                RGB::named(GREEN),
                RGB::named(BLACK),
                to_cp437('#'),
            );
        }
    }
}

impl GameState for SnakeGame {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::GameOver => self.dead(ctx),
        }
    }
}
