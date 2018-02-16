//! Controller module

use piston::input;
use piston::input::{Button, GenericEvent};

use settings::Settings;

use std;

/// Game controller
pub struct Controller {
    /// Settings defining the game board
    pub settings: Settings,
    /// Cell array
    pub cells: Vec<bool>,
    cells2: Vec<bool>,
    /// Width of the cell array
    pub size: usize,
    cursor_pos: [f64;2],
    started: bool,
}

impl Controller {
    /// Contructor
    pub fn new() -> Controller {
        let settings = Settings::new();
        let size = settings.window_size as usize / settings.cell_width as usize;
        let mut vector = Vec::with_capacity(size * size);
        for _ in 0..size*size {
            vector.push(false);
        }

        // Build glider
        vector[964] = true;
        vector[1028] = true;
        vector[1029] = true;
        vector[965] = true;
        vector[974] = true;
        vector[1038] = true;
        vector[1102] = true;
        vector[1167] = true;
        vector[1232] = true;
        vector[1233] = true;
        vector[911] = true;
        vector[848] = true;
        vector[849] = true;
        vector[1042] = true;
        vector[915] = true;
        vector[1171] = true;
        vector[980] = true;
        vector[1044] = true;
        vector[1108] = true;
        vector[1045] = true;
        vector[984] = true;
        vector[985] = true;
        vector[921] = true;
        vector[920] = true;
        vector[856] = true;
        vector[857] = true;
        vector[794] = true;
        vector[1050] = true;
        vector[796] = true;
        vector[732] = true;
        vector[1052] = true;
        vector[1116] = true;
        vector[934] = true;
        vector[870] = true;
        vector[871] = true;
        vector[935] = true;

        Controller {
            size: size,
            cells2: vector.clone(),
            cells: vector,
            cursor_pos: [0_f64; 2],
            settings: settings,
            started: false,
        }
    }

    /// Handles the computation of the next state
    pub fn compute_next_state(&mut self) {
        if !self.started {
            return;
        }
        std::mem::swap(&mut self.cells2, &mut self.cells);

        for (num, _) in self.cells2.iter().enumerate() {
            let x = num % self.size;
            let y = (num - x) / self.size;

            if self.nbr_neighbours(x, y) == 3 {
                self.cells[num] = true;
            } else if self.nbr_neighbours(x, y) > 3 || self.nbr_neighbours(x, y) < 2 {
                self.cells[num] = false;
            } else {
                self.cells[num] = self.cells2[num];
            }
        }
    }

    /// Handles Events
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if self.started {
            return;
        }

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(click) => {
                    if click == input::MouseButton::Left {
                        let x = self.cursor_pos[0] as usize / self.settings.cell_width as usize;
                        let y = self.cursor_pos[1] as usize / self.settings.cell_width as usize;
                        if self.cells[x + y * self.size] {
                            self.cells[x + y * self.size] = false;
                        } else {
                            self.cells[x + y * self.size] = true;
                        }
                        println!("{}", x + y * self.size);
                    }
                }
                Button::Keyboard(key) => {
                    if key == input::Key::Space {
                        self.started = true;
                    }
                }
                _ => {}
            }
        }
    }

    fn nbr_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        if x < self.size - 1 && self.cells2[x + y * self.size + 1] {
            count += 1;
        }
        if x > 0 && self.cells2[x + y * self.size - 1] {
            count += 1;
        }
        if y > 0 && x > 0 && self.cells2[x + (y - 1) * self.size - 1] {
            count += 1;
        }
        if y > 0 && self.cells2[x + (y - 1) * self.size] {
            count += 1;
        }
        if y > 0 && self.cells2[x + (y - 1) * self.size + 1] {
            count += 1;
        }
        if y < self.size - 1 && self.cells2[x + (y + 1) * self.size - 1] {
            count += 1;
        }
        if y < self.size - 1 && self.cells2[x + (y + 1) * self.size] {
            count += 1;
        }
        if y < self.size - 1 && x < self.size - 1 && self.cells2[x + (y + 1) * self.size + 1] {
            count += 1;
        }
        count
    }
}
