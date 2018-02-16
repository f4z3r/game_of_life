//! Root module containing the game.

use opengl_graphics::{OpenGL};
use piston_window::{WindowSettings, PistonWindow};
use piston::event_loop::{Events, EventSettings};
use piston_window;

use board::Board;

/// Game object
pub struct Game {
    board: Board,
}

impl Game {
    /// Constructor
    pub fn new() -> Game {
        Game{
            board: Board::new(),
        }
    }

    /// Run the game
    pub fn run(&mut self) {
        let opengl = OpenGL::V3_2;

        let window_settings = WindowSettings::new("The Game of Life", [self.board.controller.settings.window_size as u32; 2]).opengl(opengl).exit_on_esc(true);
        let mut window: PistonWindow = window_settings.build().expect("could not create window");

        let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut window) {
            self.board.controller.event(&e);
            window.draw_2d(&e, |context, graphics| {

                piston_window::clear([1.0; 4], graphics);
                self.board.draw(&context, graphics);
            });
        }
    }
}
