fn main(){
    let nums = vec![1, 2, 3];

    //borrowing nums
    let nums_iter = nums.iter();

    for value in nums_iter{
        println!("{}", value);
    }

    println!("{:?}", nums)

}