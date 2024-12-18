use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("整数を入力してください");
            return;
        }
    };

    for _ in 0..n {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み取りに失敗しました");

        let scores: Vec<u8> = input
            .split_whitespace()
            .filter_map(|x| x.parse::<u8>().ok())
            .collect();

        if scores.len() != 2 {
            eprintln!("整数をスペース区切りで2つ入力してください");
            continue; // 不正な入力はスキップ
        }

        let midterm_exam_score = scores[0];
        let final_exam_score = scores[1];

        let grade = calculate_grade(midterm_exam_score, final_exam_score);
        println!("{}", grade);
    }
}

fn calculate_grade(midterm_exam_score: u8, final_exam_score: u8) -> String {
    // 平均と期末試験スコアの大きい方を採用
    let average = (midterm_exam_score + final_exam_score) / 2;
    let score = std::cmp::max(average, final_exam_score);

    // 成績評価
    match score {
        90..=100 => "S".to_string(),
        80..=89 => "A".to_string(),
        70..=79 => "B".to_string(),
        60..=69 => "C".to_string(),
        _ => "F".to_string(),
    }
}
