fn main(){
    let index = find_first_a(String::from("apple"));
    match index {
        Some(value) => println!("The first 'a' is at index {}", value),
        None => println!("There is no 'a' in the string")
    }
}

fn find_first_a(s: String) -> Option<usize>{
    for (index, char) in s.chars().enumerate(){
        if char == 'a' {
            return Some(index as usize);
        }
    }
    return None;
}