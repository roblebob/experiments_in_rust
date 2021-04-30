struct Point1Typed<T> { x: T, y: T, }

struct Point2Typed<T,U> { x: T, y: U, }

impl<T> Point1Typed<T> {
    fn x(&self) -> &T { &self.x }
    fn y(&self) -> &T { &self.y }
}


fn main() {
    let p = Point1Typed { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
