struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn debug() -> u32{
        return 1;
    }
}

fn main(){
    let rect = Rect {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("The perimeter of the rectangle is {} pixels.", rect.perimeter());
    println!("checking debug {}", Rect::debug());
}