struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main(){
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("somusername123@gmail.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.username);
}