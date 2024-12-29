use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    let n: usize = input.trim().parse().expect("整数を入力してください");

    // 整数nに対して3桁ごとにカンマを挿入
    let formatted = n
        .to_string()
        .chars()
        .rev()
        .enumerate() // インデックスを取得
        .fold(String::new(), |mut acc, (i, c)| {
            if i % 3 == 0 && i != 0 {
                // 3桁ごとにカンマを挿入
                acc.push(',');
            }
            acc.push(c);
            acc
        });

    println!("{}", formatted.chars().rev().collect::<String>());
}
