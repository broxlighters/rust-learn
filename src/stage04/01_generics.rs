fn main() {
    println!("Stage 04 / Generics");
    println!("This example shows generic functions, structs, and impl blocks.\n");

    let numbers = vec![34, 50, 25, 100, 65];
    let letters = vec!['y', 'm', 'a', 'q'];

    println!("largest number = {}", largest(&numbers));
    println!("largest letter = {}", largest(&letters));

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("integer_point.x = {}", integer_point.x());
    println!("float_point.y = {}", float_point.y());

    let mixed_point = MixedPoint { x: "left", y: 9.5 };
    println!(
        "mixed point = ({}, {})",
        mixed_point.x(),
        mixed_point.y()
    );

    println!("\nTry this:");
    println!("1. Call largest with a new Vec<i32>.");
    println!("2. Change Point<T> values and re-run.");
    println!("3. Add your own generic struct or method.");
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = list
        .first()
        .expect("largest needs a non-empty slice");

    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}
