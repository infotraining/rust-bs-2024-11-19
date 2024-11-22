use bracket_lib::prelude::{main_loop, BError, BTermBuilder};

mod snake;
mod board;
mod snake_game;

fn main() -> BError {
    use snake_game::SnakeGame;

    let context = BTermBuilder::simple80x50()
        .with_title("Snake game")
        .build()?;

    let game = SnakeGame::new();

    main_loop(context, game)
}