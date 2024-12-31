use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let (x, y) = (n % 3, n % 5);

    if let (0, 0) = (x, y) {
        println!("Yes");
    } else {
        println!("{}", n);
    }
}
