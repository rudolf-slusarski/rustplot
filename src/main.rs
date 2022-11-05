use std::io;

use rustplot::{plot::Plot, render::Layout};

fn main() -> io::Result<()> {
    // println!("input equation:");
    // let mut formula = String::new();
    // stdin().read_line(&mut formula).expect("wrong input");
    let mut layout = Layout::new();
    let infimum = -350;
    let supremum = 350;
    layout = layout.add_plot(Plot::function(
        |x| x * x * 0.1,
        Some(infimum),
        Some(supremum),
    ));
    layout.save("plot.svg")
}
