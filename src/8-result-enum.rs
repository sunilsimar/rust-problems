use std::fs::read_to_string;

fn main(){
    let answer = read_from_file_sunil(String::from("a.txt"));

    match answer {
        Ok(data) => println!("{}", data),
        Err(e) => println!("{}", e),
    }
    

}

fn read_from_file_sunil(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(_e) => Err(String::from("File not found")),
    }
}