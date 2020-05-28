use crate::cell::Cell;

// The default PICO-8 palette
// https://pico-8.fandom.com/wiki/Palette
pub const DEFAULT_PALETTE: [Cell; 16] = [
    // black
    Cell::new(0, 0, 0),
    // dark-purple
    Cell::new(29, 43, 83),
    // dark-green
    Cell::new(126, 37, 83),
    // dark-green
    Cell::new(0, 135, 81),
    // brown
    Cell::new(171, 82, 54),
    // dark-gray
    Cell::new(95, 87, 79),
    // light-gray
    Cell::new(194, 195, 199),
    // white
    Cell::new(255, 241, 232),
    // red
    Cell::new(255, 0, 77),
    // orange
    Cell::new(255, 163, 0),
    // yellow
    Cell::new(255, 236, 39),
    // green
    Cell::new(0, 228, 54),
    // kblue
    Cell::new(41, 173, 255),
    // lavender
    Cell::new(131, 118, 156),
    // pink
    Cell::new(255, 119, 168),
    // light-peach
    Cell::new(255, 204, 170),
];


#[derive(Debug, Clone)]
pub struct Screen {
    pub cells: Vec<Cell>,
    pub clear: Cell,
    width: usize,
    height: usize,
    palette: [Cell;16],
}


impl Screen {
    // Create a new screen with the default palette.
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with_palette(width, height, DEFAULT_PALETTE)
    }

    // Create a new screen with a custom 16-colour palette.
    pub fn new_with_palette(width: usize, height: usize, palette: [Cell; 16]) -> Self {
        Screen {
            cells: vec![Cell::default(); (width * height) as usize],
            clear: palette[0],
            width,
            height,
            palette: palette,
        }
    }

    // Set the index of the clear colour.
    pub fn set_clear(&mut self, ix: usize) {
        self.clear = self.palette[ix];
    }

    // Clear the screen.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = self.clear;
        }
    }

    // Set an individual pixel's colour.
    pub fn set(&mut self, x: usize, y: usize, ix: usize) {
        self.cells[(y * self.width) + x] = self.palette[ix];
    }

    // Draw a filled rectangle.
    pub fn rectfill(&mut self, mut x0: usize, y0: usize, mut x1: usize, y1: usize, ix: usize) {
        if x1 < x0 {
            std::mem::swap(&mut x1, &mut x0);
        }
        for y in y0..y1 + 1 {
            for x in x0..x1 + 1 {
                self.cells[(y * self.width) + x] = self.palette[ix];
            }
        }
    }

    // Draw a filled rectangle, alternating between two colours.
    pub fn rect_alt(&mut self, mut x0: usize, y0: usize, mut x1: usize, y1: usize, a: usize, b: usize) {
        if x1 < x0 {
            std::mem::swap(&mut x1, &mut x0);
        }
        let col = [&self.palette[a], &self.palette[b]];
        for y in y0..y1 + 1 {
            if y == y0 || y == y1 {
                for x in x0..x1 + 1 {
                    self.cells[(y * self.width) + x] = *col[x & 1];
                }
            } else {
                self.cells[(y * self.width) + x0] = *col[x0 & 1];
                self.cells[(y * self.width) + x1] = *col[x1 & 1];
            }
        }
    }

    // Draw a rectangle.
    pub fn rect(&mut self, mut x0: usize, y0: usize, mut x1: usize, y1: usize, ix: usize) {
        if x1 < x0 {
            std::mem::swap(&mut x1, &mut x0);
        }
        for y in y0..y1 + 1 {
            if y == y0 || y == y1 {
                for x in x0..x1 + 1 {
                    self.cells[(y * self.width) + x] = self.palette[ix];
                }
            } else {
                self.cells[(y * self.width) + x0] = self.palette[ix];
                self.cells[(y * self.width) + x1] = self.palette[ix];
            }
        }
    }

    // Blit the screen onto a canvas.
    pub fn draw(&self, canvas: &mut [u8]) {
        self.cells
            .iter()
            .zip(canvas.chunks_exact_mut(4))
            .for_each(|(cell, pixel)| {
                pixel.copy_from_slice(&[cell.r, cell.g, cell.b, 255]);
            });
    }
}
