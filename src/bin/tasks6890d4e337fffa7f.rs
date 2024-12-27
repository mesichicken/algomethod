use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<u32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok()) // パース失敗時は無視
        .collect();

    if numbers.len() != 4 {
        eprintln!("4つの整数をスペース区切りで入力してください");
        return;
    }

    let (a, b, c, d) = (numbers[0], numbers[1], numbers[2], numbers[3]);

    // 時間の妥当性チェック
    if !(a <= b && b <= 24 && a <= c && c <= d && d <= 24) {
        eprintln!(
            "エラー: 時間は 0 <= A <= B <= 24 かつ A <= C <= D <= 24 の範囲で入力してください"
        );
        return;
    }

    // 労働時間の計算
    let total_work_time = b - a;
    let break_time = d - c;

    let effective_work_time = if total_work_time > break_time {
        total_work_time - break_time
    } else {
        0
    };

    println!("{}", effective_work_time);
}
