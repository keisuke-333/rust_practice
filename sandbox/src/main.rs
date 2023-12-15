trait Shape {
    fn calc_area(&self) -> f64;
    fn calc_perimeter(&self) -> f64;
}
struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
    fn calc_perimeter(&self) -> f64 {
        self.width + self.height
    }
}

impl Shape for Circle {
    fn calc_area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    fn calc_perimeter(&self) -> f64 {
        (self.radius + self.radius) * std::f64::consts::PI
    }
}

fn double_area(shape: &impl Shape) -> f64 {
    shape.calc_area() * 2.0
}

fn main() {
    let rectangle = Rectangle {
        width: 6.0,
        height: 6.0,
    };
    let circle = Circle { radius: 3.0 };

    println!("{}", rectangle.calc_area());
    println!("{}", rectangle.calc_perimeter());

    println!("{}", circle.calc_area());
    println!("{}", circle.calc_perimeter());

    println!("{}", double_area(&rectangle));
    println!("{}", double_area(&circle));
}
