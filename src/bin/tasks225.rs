use std::io;

fn fizz_buzz_check(number: u32) {
    let (x, y) = (number % 3, number % 5);
    match (x, y) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        _ => println!("{}", number),
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number: u32 = match input.trim().parse() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    for n in 1..=number {
        fizz_buzz_check(n);
    }
}
