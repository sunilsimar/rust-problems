use chrono::{Local, Utc};

fn main() {
    let now = Utc::now();
    println!("Current data and time in UTC: {}", now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Current data and time in UTC: {}", formatted);

    let local = Local::now();
    println!("Current data and time in Local: {}", local);

}