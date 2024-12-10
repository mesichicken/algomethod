use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let s: String = input.trim().to_string();

    if s.len() != 5 {
        eprintln!("Error: The input string must be exactly 5 characters long.");
        return;
    }

    // 中央の文字を取得
    let middle_char = s
        .chars()
        .nth(2)
        .expect("Failed to get the middle character");
    println!("{}", middle_char);
}
