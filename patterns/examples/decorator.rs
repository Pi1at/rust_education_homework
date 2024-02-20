use patterns::decorator::Circle;
use patterns::decorator::*;

fn static_decorator() {
    let decorated_circle = impl_static::Decorator::new(Square { a: 33.0 });
    decorated_circle.draw();
}

fn dynamic_decorator() {
    let circle = Circle { r: 42.0 };
    let decorated_circle = impl_dynamic::Decorator {
        shape: Box::new(circle),
        title: "Red".into(),
    };
    decorated_circle.draw();
}

fn main() {
    println!("Decorator pattern example");
    static_decorator();
    dynamic_decorator();
}
