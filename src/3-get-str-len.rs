fn main(){
    let my_string = String::from("Hello World!");
    let length = get_str_len(&my_string);
    println!("The length of the string {} is {}", my_string, length);
}

fn get_str_len(str: &String)-> usize {
    str.chars().count()
}