enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
    Square(f64),
}

fn main(){
    let rect = Shape::Rectangle(10.0, 20.0);
    let circle = Shape::Circle(10.0);
    let square = Shape::Square(10.0);

    println!("The area of the rectangle is {}", calculate_area(rect));
    println!("The area of the circle is {}", calculate_area(circle));
    println!("The area of the square is {}", calculate_area(square));
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(width, height) => width * height,
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side * side,
    }
}