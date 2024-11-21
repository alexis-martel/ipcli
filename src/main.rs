fn main() {
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";
    let image_width: i32;
    let image_height: i32;
    let image_color: bool;
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 4 {
        let w: Result<i32, _> = args[1].parse();
        let h: Result<i32, _> = args[2].parse();
        let c: Result<bool, _> = args[3]
            .to_lowercase()
            .replace("t", "true")
            .replace("f", "false")
            .parse();
        if let (Ok(w), Ok(h), Ok(c)) = (w, h, c) {
            image_width = w;
            image_height = h;
            image_color = c;
        } else {
            eprintln!(
                "{RED}{}: invalid options\n{RESET}usage: {} [w: number] [h: number] [color: {{t | f}}]",
                args[0], args[0]
            );
            panic!();
        }
    } else if args.len() == 1 {
        image_width = 10;
        image_height = 10;
        image_color = false;
    } else {
        eprintln!(
            "{RED}{}: invalid options\n{RESET}usage: {} [w: number] [h: number] [color: {{t | f}}]",
            args[0], args[0]
        );
        panic!();
    }
    let mut img = Image::new(image_width, image_height, image_color);
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
    pub fn resize(&mut self, w: i32, h: i32) {
        match w.cmp(&(self.get_width() as i32)) {
            std::cmp::Ordering::Greater => {
                // Add columns
                let width = self.get_width();
                let height = self.get_height();
                for i in 0..height {
                    println!("{}", i);
                    for _ in 0..(w - width as i32) {
                        self.grid[i].push(false);
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
                        self.grid[height + i as usize].push(false);
                    }
                }
            }
            std::cmp::Ordering::Less => {
                // Remove rows
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
        let frame_vertical: &str;
        let mut frame_horizontal = String::new();
        if frame {
            frame_vertical = "|";
            frame_horizontal += "+";
            for _ in 0..2 * self.get_width() {
                frame_horizontal += "-";
            }
            frame_horizontal += "+";
        } else {
            frame_vertical = "";
            frame_horizontal = "".to_owned();
        }
        let mut human_readable = String::new();
        human_readable += &frame_horizontal;
        human_readable += "\n";
        for line in &self.grid {
            human_readable += frame_vertical;
            for cell in line {
                if cell == &true {
                    human_readable += fill_color;
                } else {
                    human_readable += background_color;
                }
            }
            human_readable += frame_vertical;
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
            input = input.to_lowercase();
            input = input.replace(" t", " true"); // It's a hack, but it works
            input = input.replace(" f", " false");
            let command = input.split_whitespace().collect::<Vec<&str>>();
            let mut command_name = "";
            if !command.is_empty() {
                command_name = command[0].trim();
            }
            match command_name {
                "help" | "h" => {
                    self.print_help(&mut print_image);
                }
                "write" | "w" => {
                    const USAGE_MESSAGE: &str = "[x: number] [y: number] [color: {t | f}]";
                    if command.len() == 4 {
                        let x: Result<i32, _> = command[1].parse();
                        let y: Result<i32, _> = command[2].parse();
                        let c: Result<bool, _> = command[3].parse();
                        if let (Ok(x), Ok(y), Ok(c)) = (x, y, c) {
                            self.image.write_pixel(x, y, c);
                        } else {
                            self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                        }
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                    }
                }
                "fill" | "f" => {
                    const USAGE_MESSAGE: &str = "[x: number] [y: number] [color: {t | f}]";
                    if command.len() == 4 {
                        let x: Result<i32, _> = command[1].parse();
                        let y: Result<i32, _> = command[2].parse();
                        let c: Result<bool, _> = command[3].parse();
                        if let (Ok(x), Ok(y), Ok(c)) = (x, y, c) {
                            self.image.flood_fill(x, y, c);
                        } else {
                            self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                        }
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                    }
                }
                "resize" | "r" => {
                    const USAGE_MESSAGE: &str = "[w: number] [h: number]";
                    if command.len() == 3 {
                        let w: Result<i32, _> = command[1].parse();
                        let h: Result<i32, _> = command[2].parse();
                        if let (Ok(w), Ok(h)) = (w, h) {
                            self.image.resize(w, h);
                        } else {
                            self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                        }
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                    }
                }
                "clear" | "c" => {
                    const USAGE_MESSAGE: &str = "[color: {t | f}]";
                    if command.len() == 2 {
                        let c: Result<bool, _> = command[1].parse();
                        if let Ok(c) = c {
                            self.image.clear(c);
                        } else {
                            self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                        }
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                    }
                }
                "draw_rectangle" | "dr" => {
                    const USAGE_MESSAGE: &str =
                        "[x: number] [y: number] [w: number] [h: number] [color: {t | f}]";
                    if command.len() == 6 {
                        let x: Result<i32, _> = command[1].parse();
                        let y: Result<i32, _> = command[2].parse();
                        let w: Result<i32, _> = command[3].parse();
                        let h: Result<i32, _> = command[4].parse();
                        let c: Result<bool, _> = command[5].parse();
                        if let (Ok(x), Ok(y), Ok(w), Ok(h), Ok(c)) = (x, y, w, h, c) {
                            self.image.draw_rectangle(x, y, w, h, c);
                        } else {
                            self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                        }
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE, &mut print_image);
                    }
                }
                "invert" | "i" => self.image.invert(),
                "quit" | "q" => return,
                "" => print_image = false,
                _ => {
                    eprintln!("unrecognized option '{}'", command_name);
                    print_image = false;
                }
            }
        }
    }
    fn print_welcome_message(&self) {
        println!("Welcome to ipcli. Type 'help' for help. Type 'quit' to quit.");
    }
    fn print_help(&self, print_image: &mut bool) {
        println!("ipcli help:\n");
        *print_image = false;
        // TODO: Write help text
    }
    fn print_command_usage(&self, command_name: &str, usage_message: &str, print_image: &mut bool) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        eprintln!(
            "{RED}{command_name}: invalid options\n{RESET}usage: {command_name} {usage_message}"
        );
        *print_image = false;
    }
}
