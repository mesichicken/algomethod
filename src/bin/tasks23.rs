use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let s: u32 = input.trim().parse().unwrap();

    let day_hours = 24;

    let ans = day_hours - s;

    println!("{}", ans);
}
