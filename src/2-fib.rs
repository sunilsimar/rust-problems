fn main(){
    print!("{}", fib(4));
}

fn fib(num: u32) -> u32{
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 0..num-1{
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;

}