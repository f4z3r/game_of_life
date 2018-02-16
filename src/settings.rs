//! Settings module

/// Settings container
pub struct Settings {
    /// Size of game window
    pub window_size: f64,
    /// Cell width
    pub cell_width: u8,
}

impl Settings {
    /// Constructor
    ///
    /// By default build settings for a 1024x1024 pixel window with cells of width 8 pixels.
    pub fn new() -> Settings {
        Settings {
            window_size: 1024_f64,
            cell_width: 16,
        }
    }
}
