use std::io;

fn main() {
    // 1行目の入力 (文字列 S)
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let s = input.trim().to_string();

    // 2行目の入力 (正の整数 N)
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let n: usize = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input for N. Please enter a positive integer.");
            return;
        }
    };

    // エラー処理: N が範囲外の場合
    if n == 0 || n > s.len() {
        eprintln!("Error: N is out of range. Please ensure 1 <= N <= length of S.");
        return;
    }

    // N 番目の文字を取得 (1-based index)
    if let Some(ans) = s.chars().nth(n - 1) {
        println!("{}", ans);
    } else {
        eprintln!("Unexpected error: Failed to retrieve character.");
    }
}
