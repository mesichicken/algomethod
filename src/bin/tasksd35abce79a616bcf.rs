fn generate_calendar() {
    let days_in_month = 31;
    let days_in_week = 7;
    let first_day_of_month = 4; // 1 = Sunday, 2 = Monday, ..., 7 = Saturday; 3 = Wednesday

    // Print leading dots for days before the 1st
    for _ in 1..first_day_of_month {
        print!(". ");
    }

    for day in 1..=days_in_month {
        print!("{} ", day);

        // Determine if we need to move to the next line
        if (day + first_day_of_month - 1) % days_in_week == 0 {
            println!(); // Move to the next line after every week
        }
    }

    println!(); // Final newline to ensure proper formatting
}

fn main() {
    generate_calendar();
}
