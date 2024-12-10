use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let inputs: Vec<String> = input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    let s = &inputs[0];
    let t = &inputs[1];
    let u = &inputs[2];

    println!("{}{}{}", u, t, s);
}
