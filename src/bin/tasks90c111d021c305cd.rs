use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let (x, y) = (n % 3, n % 5);

    if x == 0 || y == 0 {
        println!("Yes");
    } else {
        println!("{}", n);
    }
}
