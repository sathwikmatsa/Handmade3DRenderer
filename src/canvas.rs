use super::color::*;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<Vec<Color>>,
}

impl Canvas {
    fn make_grid(w: u32, h: u32) -> Vec<Vec<Color>> {
        let mut grid = Vec::new();
        for _ in 0..h {
            let mut row = Vec::new();
            for _ in 0..w {
                row.push(Color::new(0.0, 0.0, 0.0));
            }
            grid.push(row);
        }
        grid
    }
    pub fn new(w: u32, h: u32) -> Self {
        Self{
            width: w,
            height: h,
            grid: Self::make_grid(w, h),
        }
    }
    pub fn write_pixel(&mut self, i: u32, j: u32, color: Color) {
        self.grid[i as usize][j as usize] = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_canvas() {
        let c = Canvas::new(5, 6);
        assert_eq!(c.width, 5);
        assert_eq!(c.height, 6);
        let p = Color::new(0.0, 0.0, 0.0);
        for row in c.grid.iter() {
            for pixel in row.iter() {
                assert_eq!(*pixel, p);
            }
        }
    }
    #[test]
    fn write_pixel_in_canvas() {
        let mut c = Canvas::new(5, 5);
        let pix = Color::new(1.0, 1.0, 1.0);
        c.write_pixel(1, 1, pix);
        assert_eq!(c.grid[1usize][1usize], pix);
    }
}
