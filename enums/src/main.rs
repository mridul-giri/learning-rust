enum Shape {
    Square(f64),
    Rectangle(f64, f64),
}

fn main() {
    let square = Shape::Square(6.0);
    let rectangle = Shape::Rectangle(5.0, 3.0);

    println!("{}", calculate_area(square));
    println!("{}", calculate_area(rectangle));
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Square(length) => length * length,
        Shape::Rectangle(width, height) => width * height,
    }
}
