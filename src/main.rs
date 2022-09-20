use std::io::stdin;

fn main() {
    println!("input equation:");
    let mut equation = String::new();
    stdin().read_line(&mut equation).expect("wrong input");

    println!("{}", equation);
}
