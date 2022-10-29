use std::io::stdin;

use rustplot::render::Layout;

fn main() {
    println!("input equation:");
    let mut formula = String::new();
    stdin().read_line(&mut formula).expect("wrong input");
    let x = Layout::new();
    x.save("plot.svg");
}
