use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("数値をスペース区切りで入力してください"))
        .collect();

    if numbers.len() != 4 {
        eprintln!("数値をスペース区切りで4つ入力してください");
        return;
    }

    let mut max = numbers[0];

    for num in numbers {
        if max < num {
            max = num;
        }
    }

    println!("{}", max);
}
