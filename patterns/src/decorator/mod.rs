// In object-oriented programming, the decorator pattern is a design pattern
// that allows behavior to be added to an individual object, dynamically,
// without affecting the behavior of other instances of the same class.
// The decorator pattern is often useful for adhering to the Single Responsibility Principle,
// as it allows functionality to be divided between classes with unique areas of concern
// as well as to the Open-Closed Principle, by allowing the functionality of a class to be extended without being modified.
// Decorator use can be more efficient than subclassing, because an object's behavior can be augmented without defining an entirely new object.

pub struct Circle {
    pub r: f32,
}

pub trait Shape {
    fn draw(&self);
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing Circle with radius {}", self.r);
    }
}
pub struct Square {
    pub a: f32,
}

impl Shape for Square {
    fn draw(&self) {
        println!("Drawing Square with side {}", self.a);
    }
}

pub mod impl_dynamic {
    use super::Shape;

    pub struct Decorator {
        pub shape: Box<dyn Shape>,
        pub title: String,
    }

    impl Shape for Decorator {
        fn draw(&self) {
            self.shape.draw();
            println!("[dynamic]: adding title {}", self.title);
        }
    }
}

pub mod impl_static {
    use super::{Circle, Shape};

    pub struct Decorator<T: Shape> {
        pub shape: T,
        inner_shape: Circle,
    }

    impl<T: Shape> Decorator<T> {
        pub fn new(x: T) -> Self {
            Self {
                shape: x,
                inner_shape: Circle { r: 10.0 },
            }
        }
    }

    impl<T: Shape> Shape for Decorator<T> {
        fn draw(&self) {
            self.shape.draw();
            println!(
                "[static]: adding circle, with radius {}",
                self.inner_shape.r
            );
        }
    }
}
