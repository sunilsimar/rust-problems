fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec);
    println!("{:?}", even_filter(&mut vec));

    //doing in-place
    let mut i = 0;
    while i < vec.len(){
        if vec[i] % 2 == 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
    println!("{:?}", vec);

}

fn even_filter(vec: &mut Vec<i32>) -> Vec<i32> { 
    let mut new_vec = Vec::new();
    for val in vec {
        if *val % 2 == 0 {
            new_vec.push(*val);
        }
    }

    return new_vec;
}