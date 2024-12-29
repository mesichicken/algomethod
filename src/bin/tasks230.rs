use std::io;

fn main() {
    // 標準入力からデータを受け取る
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Error reading input for N");
        return;
    }
    let n: usize = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error parsing N as usize");
            return;
        }
    };

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Error reading input for S");
        return;
    }
    let s = input.trim();
    if s.len() != n {
        eprintln!("Error: Length of S does not match N");
        return;
    }

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Error reading input for T");
        return;
    }
    let t = input.trim();
    if t.len() != n {
        eprintln!("Error: Length of T does not match N");
        return;
    }

    // SとTの異なる文字数をカウント
    let mut difference_count = 0;
    for (char_s, char_t) in s.chars().zip(t.chars()) {
        if char_s != char_t {
            difference_count += 1;
        }
    }

    // 結果を出力
    println!("{}", difference_count);
}
