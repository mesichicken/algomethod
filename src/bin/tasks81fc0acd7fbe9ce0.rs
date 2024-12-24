use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    for i in 1..=n {
        let i_string = i.to_string();
        if i_string.contains('4') || i_string.contains('9') {
            continue;
        }
        println!("{}", i);
    }
}
