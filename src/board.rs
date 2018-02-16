//! Board module

use piston_window::{Context, Graphics, Transformed, Rectangle};

use controller::Controller;

/// The game board
pub struct Board {
    /// Controller used by this game board
    pub controller: Controller,
}

impl Board {
    /// Constructor
    ///
    /// Builds a new game board.
    pub fn new() -> Board {
        Board {
            controller: Controller::new(),
        }
    }

    pub fn draw<G: Graphics>(&mut self, context: &Context, graphics: &mut G) {
        let main_border = [
            0_f64,
            0_f64,
            self.controller.settings.window_size,
            self.controller.settings.window_size,
        ];
        Rectangle::new_border([0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32], 4_f64)
            .draw(main_border,
                  &context.draw_state,
                  context.transform.trans(0_f64, 0_f64),
                  graphics);

        let cell_border = [
            0_f64,
            0_f64,
            self.controller.settings.cell_width as f64,
            self.controller.settings.cell_width as f64,
        ];

        for (num, elt) in self.controller.cells.iter().enumerate() {
            let x = num % self.controller.size;
            let y = (num - x) / self.controller.size;
            if !*elt {
                Rectangle::new_border([0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32], 0.5_f64)
                    .draw(cell_border,
                          &context.draw_state,
                          context.transform.trans(self.controller.settings.cell_width as f64 * x as f64, self.controller.settings.cell_width as f64 * y as f64),
                          graphics);
            } else {
                Rectangle::new([0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32])
                    .draw(cell_border,
                          &context.draw_state,
                          context.transform.trans(self.controller.settings.cell_width as f64 * x as f64, self.controller.settings.cell_width as f64 * y as f64),
                          graphics);
            }
        }

        self.controller.compute_next_state();
    }
}
