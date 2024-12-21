use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    let n: usize = input.trim().parse().expect("整数を入力してください");
    input.clear();

    io::stdin().read_line(&mut input).expect("入力エラー");
    let a: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("整数を入力してください"))
        .collect();

    assert_eq!(a.len(), n, "配列の長さが入力された n と一致しません");
    // aの中身を反転
    let reversed: Vec<i32> = a.into_iter().rev().collect();

    for x in reversed {
        println!("{}", x);
    }
}
