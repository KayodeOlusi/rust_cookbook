use std::f64;

fn main() {
    struct Rectangle {
        h: f64,
        b: f64,
    }

    struct Circle {
        r: f64,
    }

    trait HasArea {
        fn area(&self) -> f64;
    }

    // implement area for circle
    impl HasArea for Circle {
        fn area(&self) -> f64 {
            3.14 * (self.r * self.r)
        }
    }

    // implement area for rectangle
    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.h * self.b
        }
    }

    // variable of circle data type
    let mut circle1 = Circle { r: 10.0 };
    println!("Area of circle is: {}", circle1.area());

    // variable of rectangle data type
    let mut rect = Rectangle { h: 10.0, b: 10.0 };
    println!("Area of rectangle is: {}", rect.area());
}
