use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 50);
    scores.insert(String::from("Charlie"), 25);

    //1 Iterating over references to key-value pairs
    println!("Iterating over key-value pairs");
    for (key, value) in scores.iter(){
        println!("{}: {}", key, value);
    }

    //2 Iterating over mutable references to key-value pairs
    println!("Iterating over mutable key-value pairs");
    for (key, value) in scores.iter_mut(){
        *value += 50;
        println!("{}: {}", key, value);
    }

}
