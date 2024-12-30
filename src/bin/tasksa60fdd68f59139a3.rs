use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let s = input.trim();

    // 先頭のゼロをすべて削除
    let trimmed = s.trim_start_matches('0');

    println!("{}", trimmed);
}
