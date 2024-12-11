use std::io;

fn main() {
    let mut input = String::new();

    // 標準入力から値を読み取る
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // 入力値を整数に変換
    let s: u32 = match input.trim().parse() {
        Ok(x) if x <= 24 => x, // 24以下の正の整数であることをチェック
        _ => {
            eprintln!("Error: Please enter a valid number between 0 and 24.");
            return;
        }
    };

    // 1日の時間（定数）
    const HOURS_IN_A_DAY: u32 = 24;

    // 計算
    let remaining_hours = HOURS_IN_A_DAY - s;

    // 結果を出力
    println!("{}", remaining_hours);
}
