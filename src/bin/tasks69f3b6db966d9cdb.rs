use std::io;

fn main() {
    let n = read_line_as_string()
        .parse::<usize>()
        .expect("一行目は整数を入力してください");

    let mut pass_count = 0;

    for _ in 0..n {
        let values: Vec<i32> = read_line_as_string()
            .split_whitespace()
            .map(|x| x.parse().expect("入力値が不正です"))
            .collect();
        if values.len() != 2 {
            eprintln!("不正な入力: 必ず2つの値を入力してください");
            return;
        }
        let grade = values[0];
        let score = values[1];

        if pass_check(grade, score) {
            pass_count += 1;
        }
    }

    println!("{}", pass_count);
}

fn read_line_as_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn pass_check(grade: i32, score: i32) -> bool {
    match grade {
        1 => score >= 90,
        2 => score >= 80,
        3..=5 => score >= 70,
        _ => false,
    }
}
