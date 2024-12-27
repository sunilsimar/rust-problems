fn main(){
    let v1: Vec<i32> = vec![1, 2, 3];
    let iter = v1.iter(); 
    
    //this is not consuming it returning another iter so we can use 
    let iter2 = iter.map(|x| x + 1);
    
    for x in iter2 {
        println!("{}", x);
    }

    println!("{:?}", v1);
  }