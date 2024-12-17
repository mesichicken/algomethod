use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    const TAX_RATE: f32 = 0.1;

    let ans = n as f32 * (TAX_RATE + 1.0);
    println!("{}", ans);
}
