fn main(){
    let nums = vec![1, 2, 3];
    let iter: std::vec::IntoIter<i32> = nums.into_iter();

    for value in iter {
        println!("{}", value);
    }


    //now the owner is iter so it can't print this
    // println!("{:?}", nums)
}