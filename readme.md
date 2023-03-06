# mandelbrot-cli

`mandelbrot-cli.rs` is a command-line interface (CLI) program that displays a Mandelbrot set in your terminal. A Mandelbrot set is a mathematical fractal used to study chaos theory. A Mandelbrot set is defined as the set of all complex numbers c so that the orbit of 0 remains bounded under each iteration of the function.

The program allows you to specify certain parameters such as bounds and number of iterations through passing in arguments. For help, use `mandelbrot -h`.

## Usage
`$ mandelbrot-cli [OPTIONS]`

### Options

* `-h, --help`: Prints help information
* `-V, --version`: Prints version information
* `-m, --max_iterations <max_iterations>`: Number of iterations (default: 1000)
* `-x, --x_min <x_min>`: Minimum X value (default: -2.0)
* `-i, --x_max <x_max>`: Maximum X value (default: 1.0)
* `-y, --y_min <y_min>`: Minimum Y value (default: -1.0)
* `-j, --y_max <y_max>`: Maximum Y value (default: 1.0)
* `-w, --width <width>`: Width of generated set (default: 100)
* `-t, --height <height>`: Height of generated set (default: 24)

## Future Features

* Support for custom color schemes
* Interactive mode with zooming and panning capabilities

## Technologies Used

* Rust
* clap
* num-complex
