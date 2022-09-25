use std::io::stdin;
use rustplot::Equation;

fn main() {
    println!("input equation:");
    let mut formula = String::new();
    stdin().read_line(&mut formula).expect("wrong input");
    let mut equation = Equation::new(formula);
}
