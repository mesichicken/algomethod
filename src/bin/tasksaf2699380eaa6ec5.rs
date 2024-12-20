use std::io::stdin;

fn main() {
    // 標準入力から最初の行を取得し、n をパース
    let mut input = String::new();
    stdin().read_line(&mut input).expect("入力エラー");
    let n: usize = input.trim().parse().expect("整数を入力してください");

    // 標準入力から2行目を取得し、整数の配列をパース
    input.clear();
    stdin().read_line(&mut input).expect("入力エラー");
    let a: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("整数を入力してください"))
        .collect();

    // 配列の長さチェック
    assert_eq!(a.len(), n, "配列の長さが入力された n と一致しません");

    // 合計と平均値を計算
    let sum: i32 = a.iter().sum();
    let avg = sum / n as i32; // 整数の切り捨てを明示

    // 結果を出力
    println!("{}", avg);
}
