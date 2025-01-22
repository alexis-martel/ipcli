fn main() {
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";
    let image_width: i32;
    let image_height: i32;
    let image_color: bool;
    let mut run_script = false;
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
    } else if args.len() == 1 || (args.len() == 3 && args[1] == "-s") {
        image_width = 10;
        image_height = 10;
        image_color = false;
        if args.len() == 3 && args[1] == "-s" {
            run_script = true;
        }
    } else {
        eprintln!(
            "{RED}{}: invalid options\n{RESET}usage: {} [w: number] [h: number] [color: {{t | f}}]\n",
            args[0], args[0]
        );
        panic!();
    }
    let mut img = Image::new(image_width, image_height, image_color);
    let mut cli = Cli::new("ipcli> ".to_owned(), &mut img);
    if run_script {
        let file_path = args[2].to_owned();
        let file_contents = std::fs::read_to_string(file_path).expect("failed to read script");
        cli.parse_script(file_contents);
    }
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
        if w < 1 || h < 1 {
            eprintln!("\x1b[33mwidth or height can't be smaller than 1\x1b[0m");
            return;
        }
        match w.cmp(&(self.get_width() as i32)) {
            std::cmp::Ordering::Greater => {
                // Add columns
                let width = self.get_width();
                let height = self.get_height();
                for i in 0..height {
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
        if x < 0 || y < 0 {
            eprintln!("\x1b[33mcoordinates can't be smaller than 0\x1b[0m");
            return;
        }
        if x >= self.grid[0].len() as i32 || y >= self.grid.len() as i32 {
            return;
        }
        self.grid[y as usize][x as usize] = color;
    }
    pub fn read_pixel(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            panic!("\x1b[33mcoordinates can't be smaller than 0\x1b[0m");
        }
        self.grid[y as usize][x as usize]
    }
    pub fn flip_pixel(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 {
            eprintln!("\x1b[33mcoordinates can't be smaller than 0\x1b[0m");
            return;
        }
        let a: bool = self.grid[y as usize][x as usize];
        self.write_pixel(x, y, !a);
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
        if x < 0 || y < 0 {
            eprintln!("\x1b[33mcoordinates can't be smaller than 0\x1b[0m");
            return;
        }
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
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: bool) {
        // Compute equation
        let dy = (y2 - y1) as f32;
        let dx = (x2 - x1) as f32;
        let plot_as_y_of_x = dx > dy;
        let slope: f32 = if plot_as_y_of_x { dy / dx } else { dx / dy };
        let initial_value = if plot_as_y_of_x {
            y1 - (slope * x1 as f32) as i32
        } else {
            x1 - (slope * y1 as f32) as i32
        };
        // Plot line
        let range_start = if plot_as_y_of_x { x1 } else { y1 };
        let range_end = if plot_as_y_of_x { x2 } else { y2 };
        let range = if range_end > range_start {
            range_start..range_end + 1
        } else {
            range_end..range_start + 1
        };
        if plot_as_y_of_x {
            for x in range {
                self.write_pixel(x, (slope * x as f32 + initial_value as f32) as i32, color);
            }
        } else {
            for y in range {
                self.write_pixel((slope * y as f32 + initial_value as f32) as i32, y, color);
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn draw_curve(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: bool,
    ) {
        let x_coords = [x0, x1, x2];
        let y_coords = [y0, y1, y2];
        let start_x: i32 = x_coords.iter().min().unwrap().to_owned();
        let end_x: i32 = x_coords.iter().max().unwrap().to_owned();
        let start_y: i32 = y_coords.iter().min().unwrap().to_owned();
        let end_y: i32 = y_coords.iter().max().unwrap().to_owned();
        for x in start_x..end_x {
            for y in start_y..end_y {
                let mut is_on_curve = false;
                // Sample several t values
                for t in (0..=100).map(|x| x as f32 / 100.0) {
                    let x_curve = (1.0 - t).powi(2) * x0 as f32
                        + 2.0 * (1.0 - t) * t * x1 as f32
                        + t.powi(2) * x2 as f32;
                    let y_curve = (1.0 - t).powi(2) * y0 as f32
                        + 2.0 * (1.0 - t) * t * y1 as f32
                        + t.powi(2) * y2 as f32;
                    // Check if the pixel is close to the curve point
                    if (x_curve.round() as i32 == x) && (y_curve.round() as i32 == y) {
                        is_on_curve = true;
                        break;
                    }
                }
                if is_on_curve {
                    self.write_pixel(x, y, color);
                }
            }
        }
    }
    pub fn draw_rectangle(&mut self, x: i32, y: i32, w: i32, h: i32, color: bool) {
        if x < 0 || y < 0 {
            eprintln!("\x1b[33mcoordinates can't be smaller than 0\x1b[0m");
            return;
        }
        if w < 1 || h < 1 {
            eprintln!("\x1b[33mwidth or height can't be smaller than 1\x1b[0m");
            return;
        }
        for i in x..(x + w) {
            for j in y..(y + h) {
                self.write_pixel(i, j, color);
            }
        }
    }
    pub fn draw_rectangle_outline(&mut self, x: i32, y: i32, w: i32, h: i32, color: bool) {
        // Horizontal lines
        self.draw_line(x, y, x + w, y, color);
        self.draw_line(x, y + h, x + w, y + h, color);
        // Vertical lines
        self.draw_line(x, y, x, y + h, color);
        self.draw_line(x + w, y, x + w, y + h, color);
    }
    pub fn draw_circle(&mut self, xc: i32, yc: i32, radius: i32, color: bool) {
        if xc < 0 || yc < 0 {
            eprintln!("\x1b[33mcoordinates can't be smaller than 0\x1b[0m");
            return;
        }
        if radius < 0 {
            eprintln!("\x1b[33mradius can't be smaller than 0\x1b[0m");
            return;
        }
        // Check if the pixels in a square of side (2 * radius) are included in the circle
        let start_x = xc - radius;
        let end_x = xc + radius;
        let start_y = yc - radius;
        let end_y = yc + radius;
        for x in start_x + 1..end_x {
            for y in start_y + 1..end_y {
                // Increment start_x&y by one to correct rounding error
                if (x - xc).pow(2) + (y - yc).pow(2) < radius.pow(2) {
                    // In circle
                    self.write_pixel(x, y, color);
                }
            }
        }
    }
    pub fn draw_circle_outline(&mut self, xc: i32, yc: i32, radius: i32, color: bool) {
        if xc < 0 || yc < 0 {
            eprintln!("\x1b[33mcoordinates can't be smaller than 0\x1b[0m");
            return;
        }
        if radius < 0 {
            eprintln!("\x1b[33mradius can't be smaller than 0\x1b[0m");
            return;
        }
        // Draw a circle outline using Jesko's method
        let mut t1 = radius / 16;
        let mut x = radius;
        let mut y = 0;
        let mut t2: i32;
        while x >= y {
            self.write_pixel(xc + x, yc + y, color);
            self.write_pixel(xc - x, yc + y, color);
            self.write_pixel(xc + x, yc - y, color);
            self.write_pixel(xc - x, yc - y, color);
            self.write_pixel(yc + y, xc + x, color);
            self.write_pixel(yc + y, xc - x, color);
            self.write_pixel(yc - y, xc + x, color);
            self.write_pixel(yc - y, xc - x, color);
            y += 1;
            t1 += y;
            t2 = t1 - x;
            if t2 >= 0 {
                t1 = t2;
                x -= 1;
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
        let mut input_log: Vec<String> = vec![];
        let mut input: String;
        let mut print_image = true;
        loop {
            if print_image {
                self.image.print(true);
            }
            let old_image = self.image.grid.to_owned(); // Make a copy of the current image
            print!("{}", self.prompt_string);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            input = "".to_owned();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            self.parse_command(input, &mut input_log);
            print_image = old_image != self.image.grid; // Check wether the image has changed
        }
    }
    fn parse_command(&mut self, input: String, input_log: &mut Vec<String>) {
        let original_input = input.to_owned();
        let mut input = input;
        input = input.to_lowercase();
        input = input.replace(" t", " true"); // It's a hack, but it works
        input = input.replace(" f", " false");
        let command = input.split_whitespace().collect::<Vec<&str>>();
        let mut command_name = "";
        if !command.is_empty() {
            command_name = command[0].trim();
        }
        let mut command_ok = true;
        match command_name {
            "help" | "h" => {
                self.print_help();
                command_ok = false;
            }
            "dump" | "d" => {
                let input_log_copy = input_log.to_owned();
                for command in input_log_copy {
                    println!("{};", command.trim());
                }
                command_ok = false;
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
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
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
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
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
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
                }
            }
            "clear" | "c" => {
                const USAGE_MESSAGE: &str = "[color: {t | f}]";
                if command.len() == 2 {
                    let c: Result<bool, _> = command[1].parse();
                    if let Ok(c) = c {
                        self.image.clear(c);
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
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
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
                }
            }
            "draw_rectangle_outline" | "dro" => {
                const USAGE_MESSAGE: &str =
                    "[x: number] [y: number] [w: number] [h: number] [color: {t | f}]";
                if command.len() == 6 {
                    let x: Result<i32, _> = command[1].parse();
                    let y: Result<i32, _> = command[2].parse();
                    let w: Result<i32, _> = command[3].parse();
                    let h: Result<i32, _> = command[4].parse();
                    let c: Result<bool, _> = command[5].parse();
                    if let (Ok(x), Ok(y), Ok(w), Ok(h), Ok(c)) = (x, y, w, h, c) {
                        self.image.draw_rectangle_outline(x, y, w, h, c);
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
                }
            }
            "draw_line" | "dl" => {
                const USAGE_MESSAGE: &str =
                    "[x1: number] [y1: number] [x2: number] [y2: number] [color: {t | f}]";
                if command.len() == 6 {
                    let x1: Result<i32, _> = command[1].parse();
                    let y1: Result<i32, _> = command[2].parse();
                    let x2: Result<i32, _> = command[3].parse();
                    let y2: Result<i32, _> = command[4].parse();
                    let c: Result<bool, _> = command[5].parse();
                    if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2), Ok(c)) = (x1, y1, x2, y2, c) {
                        self.image.draw_line(x1, y1, x2, y2, c);
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
                }
            }
            "draw_curve" | "db" => {
                const USAGE_MESSAGE: &str =
                    "[x0: number] [y0: number] [x1: number] [y1: number] [x2: number] [y2: number] [color: {t | f}]";
                if command.len() == 8 {
                    let x0: Result<i32, _> = command[1].parse();
                    let y0: Result<i32, _> = command[2].parse();
                    let x1: Result<i32, _> = command[3].parse();
                    let y1: Result<i32, _> = command[4].parse();
                    let x2: Result<i32, _> = command[5].parse();
                    let y2: Result<i32, _> = command[6].parse();
                    let c: Result<bool, _> = command[7].parse();
                    if let (Ok(x0), Ok(y0), Ok(x1), Ok(y1), Ok(x2), Ok(y2), Ok(c)) =
                        (x0, y0, x1, y1, x2, y2, c)
                    {
                        self.image.draw_curve(x0, y0, x1, y1, x2, y2, c);
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
                }
            }
            "draw_circle" | "dc" => {
                const USAGE_MESSAGE: &str =
                    "[x: number] [y: number] [radius: number] [color: {t | f}]";
                if command.len() == 5 {
                    let x: Result<i32, _> = command[1].parse();
                    let y: Result<i32, _> = command[2].parse();
                    let r: Result<i32, _> = command[3].parse();
                    let c: Result<bool, _> = command[4].parse();
                    if let (Ok(x), Ok(y), Ok(r), Ok(c)) = (x, y, r, c) {
                        self.image.draw_circle(x, y, r, c);
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
                }
            }
            "draw_circle_outline" | "dco" => {
                const USAGE_MESSAGE: &str =
                    "[x: number] [y: number] [radius: number] [color: {t | f}]";
                if command.len() == 5 {
                    let x: Result<i32, _> = command[1].parse();
                    let y: Result<i32, _> = command[2].parse();
                    let r: Result<i32, _> = command[3].parse();
                    let c: Result<bool, _> = command[4].parse();
                    if let (Ok(x), Ok(y), Ok(r), Ok(c)) = (x, y, r, c) {
                        self.image.draw_circle_outline(x, y, r, c);
                    } else {
                        self.print_command_usage(command_name, USAGE_MESSAGE);
                        command_ok = false;
                    }
                } else {
                    self.print_command_usage(command_name, USAGE_MESSAGE);
                    command_ok = false;
                }
            }
            "invert" | "i" => self.image.invert(),
            "quit" | "q" => std::process::exit(0),
            "" => command_ok = false,
            _ => {
                eprintln!("unrecognized command '{}'", command_name);
                command_ok = false;
            }
        }
        if command_ok {
            input_log.push(original_input);
        }
    }
    fn parse_script(&mut self, script_text: String) {
        println!("Running script…");
        for command in script_text.split(";") {
            self.parse_command(command.to_owned(), &mut vec![]);
        }
        println!("Done running script");
    }
    fn print_welcome_message(&self) {
        println!("Welcome to ipcli. Type 'help' for help. Type 'quit' to quit.");
    }
    fn print_help(&self) {
        const HELP_TEXT: &str = "\x1b[1mIMAGE PAINT COMMAND LINE INTERFACE\x1b[0m
    Manipulate one-bit bitmap graphics from the command-line.

\x1b[1mUSAGE\x1b[0m
    ipcli [w: number] [h: number] [color: {t | f}]
        Creates a new image of the specified dimensions and color.
    ipcli -s [path: file path]
        Generates the image from the script at `path` and start the IPCLI.
    
\x1b[1mCOMMANDS\x1b[0m
    help               | h: Prints this message;
    write [x] [y] [c]  | w: Sets the pixel at (x, y) to color `c`;
    fill [x] [y] [c]   | f: Flood fills from (x, y) with color `c`;
    resize [w] [h]     | r: Resizes the image to `w` * `h`;
    clear [c]          | c: Fills the image with color `c`;
    invert             | i: Inverts the image;
    quit               | q: Exits the program;
    ---
    draw_rectangle [x] [y] [w] [h] [c]           | dr: Draws a `w` * `h` rectangle of color `c` at (x, y);
    draw_line [x1] [y1] [x2] [y2] [c]            | dl: Draws a line of color `c` from (x1, y1) to (x2, y2);
    draw_curve [x0] [y0] [x1] [y1] [x2] [y2] [c] | db: Draws a quadratic Bézier curve with control points (x0, y0)(x1, y1), (x2, y2) with color `c`;
    draw_circle [x] [y] [r] [c]                  | dc: Draws a circle of radius `r` with centre (x, y) with colo`c`;
    ---
    draw_rectangle_outline [x] [y] [w] [h] [c] | dro: Draws the outline of a `w` * `h` rectangle at (x, y) witcolor `c`;
    draw_circle_outline [x] [y] [r] [c]        | dco: Draws the outline of a circle of radius `r` with centre (x,  with color `c`;
    ---
    dump | d: Dumps all executed commands as a script to stdout.

\x1b[1mABBREVIATIONS\x1b[0m
    x: x-coordinate (must be positive or zero);
    y: y-coordinate (must be positive or zero);
    w: width        (must be positive or zero);
    h: height       (must be positive or zero);
    r: radius       (must be positive or zero);
    c: color        (must be either `t` or `f`);
    ---
    t: shorthand for `true`;
    f: shorthand for `false`.
    
\x1b[1mSCRIPTING\x1b[0m
    The IPCLI supports basic scripting. Scripts are plain text files (the `.ipcli` extension is recommended) containing a list of commands to execute, separated by semicolons. See the documentation on the `dump` command and the `-s` option. Sample scripts are available in the source repository.";
        println!("{}", HELP_TEXT);
    }
    fn print_command_usage(&self, command_name: &str, usage_message: &str) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        eprintln!(
            "{RED}{command_name}: invalid options\n{RESET}usage: {command_name} {usage_message}"
        );
    }
}
