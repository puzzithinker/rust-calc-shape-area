//Define the trait of the Area
trait AreaFunction {
    fn area(&self) -> f64;
}


//Define the struct of the Shapes
#[derive(Debug)]
struct RightTriangle {
    base: f64,
    height: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}



impl RightTriangle {
    fn new(base: f64, height: f64) -> Self {
        RightTriangle {
            base,
            height,
        }
    }
}
impl AreaFunction for RightTriangle {
    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle {
            width,
            height,
        }
    }
}
impl AreaFunction for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}


impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}





impl AreaFunction for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }
}



fn print_area<T: AreaFunction + std::fmt::Debug>(s: &T) {
    println!("The measurment of {:?} and the area is {}", s, s.area());
}

//Define the main
fn main() {
    let shape1 = RightTriangle::new(2.0, 3.0);
    let shape2 = Rectangle::new(3.0, 4.0);
    let shape3 = Circle::new(10.0);
    print_area(&shape1);
    print_area(&shape2);
    print_area(&shape3);
}
