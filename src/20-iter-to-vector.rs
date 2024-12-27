fn main(){
    let v1 = vec![1, 2, 3, 4, 5];
    let iter = v1.iter().filter(|x| *x % 2 == 1).map(|x| x * 2);

    let v2: Vec<i32> = iter.collect();

    println!("{:?}", v2);
}