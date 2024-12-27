fn main(){
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    //take the ownership of v1_iter
    let sum: i32 = v1_iter.sum();

    println!("The sum is {}", sum);

    //can't use v1_iter because it's now owned by sum
    // for val in v1_iter{
    //     println!("{}", val);
    // }

}