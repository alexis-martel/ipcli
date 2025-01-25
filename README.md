# IPCLI

## Image Paint Command Line Interface

An interactive command line tool to manipulate one-bit bitmap graphics! Built in Rust.

## To Run

1. Clone this repo;
2. `cargo run`!

> [!IMPORTANT]
> Binaries for macOS (they might work on Linux) are available in [GitHub Releases](https://github.com/alexis-martel/ipcli/releases).

## Help

To get help, run IPCLI and type `help`, then press enter.

> [!TIP]
> Type `h` instead to get help faster!

The output should be:

<pre>
<b>IMAGE PAINT COMMAND LINE INTERFACE</b>
    Manipulate one-bit bitmap graphics from the command-line.

<b>USAGE</b>
    ipcli [w: number] [h: number] [color: {t | f}]
        Creates a new image of the specified dimensions and color.
    ipcli -s [path: file path]
        Generates the image from the script at `path` and start the IPCLI.
    
<b>COMMANDS</b>
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
    draw_curve [x0] [y0] [x1] [y1] [x2] [y2] [c] | db: Draws a quadratic Bézier curve with control points (x0, y0), (x1, y1), (x2, y2) with color `c`;
    draw_circle [x] [y] [r] [c]                  | dc: Draws a circle of radius `r` with centre (x, y) with color `c`;
    ---
    draw_rectangle_outline [x] [y] [w] [h] [c] | dro: Draws the outline of a `w` * `h` rectangle at (x, y) with color `c`;
    draw_circle_outline [x] [y] [r] [c]        | dco: Draws the outline of a circle of radius `r` with centre (x, y) with color `c`;
    ---
    dump | d: Dumps all executed commands as a script to stdout.

<b>ABBREVIATIONS</b>
    x: x-coordinate (must be positive or zero);
    y: y-coordinate (must be positive or zero);
    w: width        (must be positive or zero);
    h: height       (must be positive or zero);
    r: radius       (must be positive or zero);
    c: color        (must be either `t` or `f`);
    ---
    t: shorthand for `true`;
    f: shorthand for `false`.
    
<b>SCRIPTING</b>
    The IPCLI supports basic scripting. Scripts are plain text files (the `.ipcli` extension is recommended) containing a list of commands to execute, separated by semicolons. See the documentation on the `dump` command and the `-s` option. Sample scripts are available in the source repository.
</pre>

## Demo

[Demo video on YouTube](https://youtu.be/izrNcMY8iaE).

## Roadmap

- [x] Add a `draw_line` command;
- [x] Add a `draw_circle` command;
- [x] Add a `draw_curve` command;
- [x] Add scripting support.
