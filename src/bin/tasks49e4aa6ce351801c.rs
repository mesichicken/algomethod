fn generate_calendar() {
    let days_in_month = 31;
    let days_in_week = 7;

    for day in 1..=days_in_month {
        print!("{} ", day);
        if day % days_in_week == 0 {
            println!(); // Move to the next line after every week
        }
    }

    println!(); // Final newline to ensure proper formatting
}

fn main() {
    generate_calendar();
}
