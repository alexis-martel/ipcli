# IPCLI

## Image Paint Command Line Interface

An interactive command line tool to manipulate one-bit bitmap graphics! Built in Rust.

## To Run

1. Clone this repo;
2. `cargo run`!

> [!IMPORTANT]
> You need a Rust compiler to run IPCLI. No binaries are available!

## Help

To get help, run IPCLI and type `help`, then press enter.

> [!TIP]
> Type `h` instead to get help faster!

The output should be:

<pre>
Manipulate one-bit bitmap graphics from the command-line.

<b>USAGE</b>
    ipcli [w: number] [h: number] [color: {t | f}]
        Creates a new image of the specified dimensions and color.
    
<b>COMMANDS</b>
    help               | h: Prints this message;
    write [x] [y] [c]  | w: Sets the pixel at (x, y) to color `c`;
    fill [x] [y] [c]   | f: Flood fills from (x, y) with color `c`;
    resize [w] [h]     | r: Resizes the image to `w` * `h`;
    clear [c]          | c: Fills the image with color `c`;
    invert             | i: Inverts the image;
    quit               | q: Exits the program;
    ---
    draw_rectangle [x] [y] [w] [h] [c] | dr: Draws a `w` * `h` rectangle of color `c` at (x, y);
    draw_line [x1] [y1] [x2] [y2] [c]  | dl: Draws a line of color `c` from (x1, y1) to (x2, y2);
    draw_circle [x] [y] [r] [c]        | dc: Draws a circle of radius `r` with centre (x, y);
    ---
    draw_rectangle_outline [x] [y] [w] [h] [c] | dro: Draws the outline of a `w` * `h` rectangle at (x, y) with color `c`;
    draw_circle_outline [x] [y] [r] [c]        | dco: Draws the outline of a circle of radius `r` with centre (x, y).

<b>ABBREVIATIONS USED</b>
    x: x-coordinate (must be positive or zero);
    y: y-coordinate (must be positive or zero);
    w: width        (must be positive or zero);
    h: height       (must be positive or zero);
    r: radius       (must be positive or zero);
    c: color        (must be either `t` or `f`);
    ---
    t: shorthand for `true`;
    f: shorthand for `false`.
</pre>

## Demo

TODO: Make a demo video.

## Roadmap

- [x] Add a `draw_line` command;
- [x] Add a `draw_circle` command;
- [ ] Add a `draw_curve` command;
- [ ] Add scripting support.
