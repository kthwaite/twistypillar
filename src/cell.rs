#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

impl Cell {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Cell { r, g, b }
    }
}
