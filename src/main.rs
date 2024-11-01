fn main() {
    let mut img = Image::new(50, 50, false);
    let a = img.get_height();
    let b = img.get_width();
    println!("{}, {}", a, b);
    img.draw_rectangle(1, 1, 5, 3, true);
    img.draw_rectangle(1, 5, 7, 4, true);
    img.draw_rectangle(7, 1, 2, 2, true);
    img.draw_rectangle(10, 0, 40, 48, true);
    img.flip_pixel(3, 4);
    img.invert();
    img.flip_pixel(8, 5);
    img.flip_pixel(9, 5);
    println!("--BEFORE--");
    img.print();
    println!("--AFTER--");
    img.flood_fill(3, 3, true);
    img.print();
    img.clear(true);
    img.print();
}

struct Image {
    grid: Vec<Vec<bool>>,
}

impl Image {
    pub fn new(width: i32, height: i32, color: bool) -> Image {
        if width <= 0 || height <= 0 {
            panic!("at least one dimension of the image is smaller than 1");
        }
        let mut grid: Vec<Vec<bool>> = vec![];
        for i in 0..height {
            grid.push(vec![]);
            for _ in 0..width {
                grid[i as usize].push(color);
            }
        }
        Image { grid }
    }
    pub fn get_width(&self) -> usize {
        self.grid[0].len()
    }
    pub fn get_height(&self) -> usize {
        self.grid.len()
    }
    pub fn write_pixel(&mut self, x: i32, y: i32, color: bool) {
        self.grid[y as usize][x as usize] = color;
    }
    pub fn read_pixel(&self, x: i32, y: i32) -> bool {
        self.grid[y as usize][x as usize]
    }
    pub fn flip_pixel(&mut self, x: i32, y: i32) {
        let a: bool = self.grid[y as usize][x as usize];
        self.grid[y as usize][x as usize] = !a;
    }
    pub fn get_pixel_coordinates(&self) -> Vec<(i32, i32)> {
        let mut pixel_coordinates: Vec<(i32, i32)> = Vec::new();
        for (y, line) in self.grid.iter().enumerate() {
            for (x, _pixel) in line.iter().enumerate() {
                pixel_coordinates.push((x as i32, y as i32));
            }
        }
        pixel_coordinates
    }
    pub fn invert(&mut self) {
        for (x, y) in self.get_pixel_coordinates() {
            self.flip_pixel(x, y);
        }
    }
    pub fn flood_fill(&mut self, x: i32, y: i32, color: bool) {
        if self.read_pixel(x, y) == color {
            // Fill colour is the same as existent colour
            return;
        }
        fn flood_helper(img: &mut Image, x: i32, y: i32, color: bool) {
            if x as usize > img.grid[0].len() - 1 || y as usize > img.grid.len() - 1 {
                // Exit if out of bounds
                return;
            }
            if img.read_pixel(x, y) == color {
                // Exit if pixel colour is the same as fill colour
                return;
            }
            img.write_pixel(x, y, color);
            // Propagate to neighbouring pixels
            flood_helper(img, x + 1, y, color);
            flood_helper(img, x - 1, y, color);
            flood_helper(img, x, y + 1, color);
            flood_helper(img, x, y - 1, color);
        }
        flood_helper(self, x, y, color);
    }
    pub fn clear(&mut self, color: bool) {
        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                self.write_pixel(x as i32, y as i32, color);
            }
        }
    }
    pub fn get_human_readable(&self, fill_color: &str, background_color: &str) -> String {
        let mut human_readable = String::new();
        for line in &self.grid {
            for cell in line {
                if cell == &true {
                    human_readable += fill_color;
                } else {
                    human_readable += background_color;
                }
            }
            human_readable += "\n";
        }
        human_readable
    }
    pub fn print(&self) {
        println!("{}", self.get_human_readable("█ ", "  "));
    }
    pub fn draw_rectangle(&mut self, x: i32, y: i32, w: i32, h: i32, color: bool) {
        for i in x..(x + w) {
            for j in y..(y + h) {
                self.write_pixel(i, j, color);
            }
        }
    }
}

struct CLI {
    prompt: str,
}
