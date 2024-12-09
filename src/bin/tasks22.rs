use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let s: String = input.trim().parse().unwrap();

    println!("{}", s.chars().nth(2).unwrap());
}
