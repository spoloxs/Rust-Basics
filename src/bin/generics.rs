struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // This is a generic as T
    fn mixup<Y, W>(self, other: Point<Y, W>) -> Point<U, Y> {
        // Here T is the type that will taken as an arg from the user
        Point{
            x: self.y,
            y: other.x,
        }
    }
}

fn main() {
    let p1 = Point{x: 1, y: 10.5};
    let p2 = Point{x: 1.3, y: 22};
    let p3 = p1.mixup(p2);
    println!("x: {}, y: {}", p3.x, p3.y);
}
