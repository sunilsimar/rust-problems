fn main(){
    let nums = vec![1, 2, 3];
    let mut iter = nums.iter();

    while let Some(val) = iter.next() {
        println!("{}", val);
    }
}