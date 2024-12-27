use std::vec;

fn main(){
    let mut v1 = vec![1, 2, 3];
    let iter = v1.iter_mut();

    for value in iter {
        *value = *value + 1;
    } 

    println!("{:?}", v1)
}