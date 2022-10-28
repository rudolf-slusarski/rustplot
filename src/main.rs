use std::io::stdin;

fn main() {
    println!("input equation:");
    let mut formula = String::new();
    stdin().read_line(&mut formula).expect("wrong input");
}
