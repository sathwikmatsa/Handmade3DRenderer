use super::color::*;
use std::io::prelude::*;
use std::fs::File;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<Color>,
}

impl Canvas {
    fn make_grid(w: u32, h: u32) -> Vec<Color> {
        let mut grid = Vec::new();
        for _ in 0..h*w {
            grid.push(Color::new(0.0, 0.0, 0.0));
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
    pub fn pixel_at(&self, row: u32, col: u32) -> Color {
        assert!(row < self.height, "row value {} overflows height of canvas", row);
        assert!(col < self.width, "col value {} overflows width of canvas", col);
        self.grid[(row*self.width + col) as usize]
    }
    pub fn set_pixel(&mut self, row: u32, col: u32, color: Color) {
        assert!(row < self.height, "row value {} overflows height of canvas", row);
        assert!(col < self.width, "col value {} overflows width of canvas", col);
        self.grid[(row*self.width + col) as usize] = color
    }
    fn clamp(i: f32) -> u8 {
        let mut scaled = (i * 255.0) as i32;
        if scaled > 255 {
            scaled = 255;
        } else if scaled < 0 {
            scaled = 0;
        }
        scaled as u8
    }
    pub fn save_as_ppm(&self, filename: &str) {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
        for pixel in self.grid.iter() {
            ppm += format!("{} {} {}\n", Self::clamp(pixel.red), Self::clamp(pixel.green), Self::clamp(pixel.blue)).as_str();
        }
        let mut file = File::create(filename).expect("Unable to create file");
        file.write(ppm.as_bytes()).expect("Unable to write to file");
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
        for pixel in c.grid.iter() {
            assert_eq!(*pixel, p);
        }
    }
    #[test]
    fn set_pixel_in_canvas() {
        let mut c = Canvas::new(5, 5);
        let pix = Color::new(1.0, 1.0, 1.0);
        c.set_pixel(1, 1, pix);
        assert_eq!(c.pixel_at(1, 1), pix);
    }
    #[test]
    fn canvas_to_ppm() {
        let mut c = Canvas::new(2, 2);
        let p0 = Color::new(1.0, 1.0, 1.0);
        let p1 = Color::new(-1.0, -1.0, -1.0);
        let p2 = Color::new(2.0, 2.0, 2.0);
        let p3 = Color::new(0.5, 0.5, 0.5);
        c.set_pixel(0, 0, p0);
        c.set_pixel(0, 1, p1);
        c.set_pixel(1, 0, p2);
        c.set_pixel(1, 1, p3);
        c.save_as_ppm("img.ppm");

        let ppm = format!("P3\n{} {}\n255\n255 255 255\n0 0 0\n255 255 255\n127 127 127\n", c.width, c.height);
        let mut file = File::open("img.ppm").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        std::fs::remove_file("img.ppm").expect("Unable to remove file img.ppm");

        assert_eq!(contents, ppm);
    }
}
