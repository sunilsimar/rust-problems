use std::collections::HashMap;

fn main(){
    let mut users = HashMap::new();

    users.insert(String::from("Sunil"), 21);
    users.insert(String::from("Kartik"), 22);

    let first_users_age = users.get("Sunil");

    match first_users_age {
        Some(age) => println!("First user age is {}", age),
        None => println!("User not found"),
    }
}