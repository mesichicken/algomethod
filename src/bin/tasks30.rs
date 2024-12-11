use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if numbers.len() != 2 {
        println!("エラー");
        return;
    }

    let a = numbers[0];
    let b = numbers[1];

    let a_ones = a % 10;
    let b_ones = b % 10;

    if a_ones < b_ones {
        println!("{}", a);
    } else {
        println!("{}", b);
    }
}
