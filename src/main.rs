use std::io;

use rustplot::{plot::Plot, render::Layout};

fn main() -> io::Result<()> {
    // println!("input equation:");
    // let mut formula = String::new();
    // stdin().read_line(&mut formula).expect("wrong input");
    let range = (-350., 350.);
    let mut layout = Layout::new();
    let cube_function = Plot::function(|x| x * x * x, range.0, range.1, 0.1);
    layout.set_range(range);
    layout.add_plot(cube_function);
    layout.save("plot.svg")
}
