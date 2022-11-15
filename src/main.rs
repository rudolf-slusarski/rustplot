use std::io;

use rustplot::render::Layout;

fn main() -> io::Result<()> {
    // println!("input equation:");
    // let mut formula = String::new();
    // stdin().read_line(&mut formula).expect("wrong input");
    let range = (600, 4000);
    let layout = Layout::new(range);
    // let cube_function = Plot::function(|x| x * x, range.0, range.1, 0.02);

    layout.save("plot.svg")
}
