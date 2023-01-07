pub mod sample_trait {
    pub trait Shape1 {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn default_something(&self) -> &str {
            "This is default method!"
        }
        fn do_something();
    }

    pub struct Rectangle1 {
        pub width: f64,
        pub height: f64,
    }

    impl Shape1 for Rectangle1 {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }
        fn calc_perimeter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }
        fn default_something(&self) -> &str {
            "This is Rectangle1 default!"
        }
        fn do_something() {
            println!("This is Rectangle function")
        }
    }

    pub struct Circle1 {
        pub radius: f64,
    }

    impl Shape1 for Circle1 {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }
        fn calc_perimeter(&self) -> f64 {
            self.radius * 2.0 * std::f64::consts::PI
        }
        fn do_something() {
            println!("This is Circle function")
        }
    }

    pub fn double_area(shape: &impl Shape1) -> f64 {
        shape.calc_area() * 2.0
    }
}
