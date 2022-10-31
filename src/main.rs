use std::io::{self, stdin};

use rustplot::render::Layout;

fn main() -> io::Result<()> {
    // println!("input equation:");
    // let mut formula = String::new();
    // stdin().read_line(&mut formula).expect("wrong input");
    let x = Layout::new();
    x.save("plot.svg")
}
