fn main() {
    let mut img = Image::new(10, 10, false);
    let mut cli = Cli::new("ipcli> ".to_owned(), &mut img);
    cli.start();
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
    fn get_width(&self) -> usize {
        self.grid[0].len()
    }
    fn get_height(&self) -> usize {
        self.grid.len()
    }
    pub fn resize(&mut self, w: i32, h: i32, color: bool) {
        match w.cmp(&(self.get_width() as i32)) {
            std::cmp::Ordering::Greater => {
                // Add columns
                let width = self.get_width();
                let height = self.get_height();
                for i in 0..height {
                    println!("{}", i);
                    for _ in 0..(w - width as i32) {
                        self.grid[i].push(color);
                    }
                }
            }
            std::cmp::Ordering::Less => {
                // Remove columns
                let width = self.get_width();
                let height = self.get_height();
                for i in 0..height {
                    for _ in 0..(width as i32 - w) {
                        self.grid[i].pop();
                    }
                }
            }
            std::cmp::Ordering::Equal => {}
        }
        match h.cmp(&(self.get_height() as i32)) {
            std::cmp::Ordering::Greater => {
                // Add rows
                let width = self.get_width();
                let height = self.get_height();
                for i in 0..(h - height as i32) {
                    self.grid.push(Vec::new());
                    for _ in 0..width {
                        self.grid[height + i as usize].push(color);
                    }
                }
            }
            std::cmp::Ordering::Less => {
                // Remove rows
                let width = self.get_width();
                let height = self.get_height();
                for _ in 0..(height as i32 - h) {
                    self.grid.pop();
                }
            }
            std::cmp::Ordering::Equal => {}
        }
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
    fn get_human_readable(&self, fill_color: &str, background_color: &str, frame: bool) -> String {
        let mut frame_vertical = String::new();
        let mut frame_horizontal = String::new();
        if frame {
            frame_vertical = "|".to_owned();
            frame_horizontal += "+";
            for _ in 0..2 * self.get_width() {
                frame_horizontal += "-";
            }
            frame_horizontal += "+";
        } else {
            frame_vertical = "".to_owned();
            frame_horizontal = "".to_owned();
        }
        let mut human_readable = String::new();
        human_readable += &frame_horizontal;
        human_readable += "\n";
        for line in &self.grid {
            human_readable += &frame_vertical;
            for cell in line {
                if cell == &true {
                    human_readable += fill_color;
                } else {
                    human_readable += background_color;
                }
            }
            human_readable += &frame_vertical;
            human_readable += "\n";
        }
        human_readable += &frame_horizontal;
        human_readable
    }
    pub fn print(&self, frame: bool) {
        println!("{}", self.get_human_readable("██", "  ", frame));
    }
    pub fn draw_rectangle(&mut self, x: i32, y: i32, w: i32, h: i32, color: bool) {
        for i in x..(x + w) {
            for j in y..(y + h) {
                self.write_pixel(i, j, color);
            }
        }
    }
}

struct Cli<'cli_lifetime> {
    prompt_string: String,
    image: &'cli_lifetime mut Image,
}

impl<'cli_lifetime> Cli<'cli_lifetime> {
    pub fn new(prompt_string: String, image: &mut Image) -> Cli {
        Cli {
            prompt_string,
            image,
        }
    }
    pub fn start(&mut self) {
        self.print_welcome_message();
        let mut input: String;
        let mut print_image = true;
        loop {
            if print_image {
                self.image.print(true);
            }
            print_image = true;
            print!("{}", self.prompt_string);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            input = "".to_owned();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let command = input.split(" ").collect::<Vec<&str>>();
            let command_name = command[0].trim();
            match command_name {
                "help" => {
                    self.print_help();
                    print_image = false;
                }
                "write" | "w" => println!("d"),
                "clear" | "c" => self.image.clear(true),
                "quit" | "q" => return,
                _ => {
                    println!("Unrecognized option '{}'", command_name);
                    print_image = false;
                }
            }
        }
    }
    fn print_welcome_message(&self) {
        println!("Welcome to ipcli. Type 'help' for help.");
    }
    fn print_help(&self) {
        println!("ipcli help:\n");
        // TODO: Write help text
    }
}
