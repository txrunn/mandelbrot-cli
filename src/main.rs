use clap::Parser;
use num::complex::Complex;

#[derive(Parser, Debug)]
/// Displays a Mandelbrot set in your terminal.A Mandelbrot set is a mathematical fractal used to study chaos theory. A Mandelbrot set is defined as the set of all complex numbers c so that the orbit of 0 remains bounded under each iteration of the function. Certain parameters of the program such as bounds and number of iterations can be defined through passing in arguments (for help use "mandelbrot -h").
#[clap(author, version, about)]
struct Args {
   /// Number of iterations
   #[clap(short, long, value_parser, default_value_t = 1000)]
   max_iterations: usize,

   /// Minimum X value
   #[clap(short, long, value_parser, default_value_t = -2.0)]
   x_min: f64,

   /// Maximum X value
   #[clap(short = 'i', long, value_parser, default_value_t = 1.0)]
   x_max: f64,

   /// Minimum Y value
   #[clap(short, long, value_parser, default_value_t = -1.0)]
   y_min: f64,

   /// Maximum Y value
   #[clap(short = 'j', long, value_parser, default_value_t = 1.0)]
   y_max: f64,

   /// Width of generated set
   #[clap(short, long, value_parser, default_value_t = 100)]
   width: usize,

   /// Height of generated set
   #[clap(short = 't', long, value_parser, default_value_t = 24)]
   height: usize,
}

fn calculate_mandelbrot (max_iters: usize, x_min: f64, x_max: f64, y_min: f64,y_max: f64, width: usize, height: usize,) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);

    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point (cx: f64, cy: f64, max_iters: usize,) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(values: Vec<Vec<usize>>) {
    for row in values {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let args = Args::parse();
    let mandelbrot = calculate_mandelbrot(args.max_iterations, args.x_min, args.x_max, args.y_min, args.y_max, args.width, args.height);

    render_mandelbrot(mandelbrot);
}