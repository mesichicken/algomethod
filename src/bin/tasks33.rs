use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin()
        .read_line(&mut input1)
        .expect("文字列の読み取りに失敗しました");
    io::stdin()
        .read_line(&mut input2)
        .expect("数値の読み取りに失敗しました");

    let s: String = input1.trim().to_string();

    let numbers: Vec<usize> = input2
        .split_whitespace()
        .map(|x| x.parse().expect("無効な数値が入力されました"))
        .collect();

    if numbers.len() != 2 {
        eprintln!("数値をスペース区切りで2つ入力してください");
        return;
    }

    let mut char_vec: Vec<char> = s.chars().collect();

    let a = numbers[0] - 1;
    let b = numbers[1] - 1;

    if a >= char_vec.len() || b >= char_vec.len() {
        eprintln!("インデックスが文字列の範囲外です");
        return;
    }

    char_vec.swap(a, b);

    let ans: String = char_vec.into_iter().collect();
    println!("{}", ans);
}
