fn main() {
    // create a struct
    struct Circle {
        x: f64,
        radius: f64,
    }

    // methods for the struct
    impl Circle {
        // pub makes the method public which makes it accessible outside the scope
        pub fn get_x(&self) -> f64 {
            self.x
        }
    }

    // create a struct variable
    let mut circle1 = Circle {
        x: 10.0,
        radius: 10.0,
    };

    println!("x: {}, radius: {}", circle1.x, circle1.radius);
    println!("x: {}", circle1.get_x());
}
