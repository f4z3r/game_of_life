//! An implementation of Conway's Game of Life.

#[deny(missing_docs)]

extern crate piston;
extern crate piston_window;
extern crate opengl_graphics;

mod game;
mod settings;
mod controller;
mod board;

use game::Game;

fn main() {
    Game::new().run();
}
