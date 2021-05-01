struct Point1Typed<T> { x: T, y: T, }

struct Point2Typed<T,U> { x: T, y: U, }

impl<T> Point1Typed<T> {
    fn x(&self) -> &T { 
        &self.x 
    }
    fn y(&self) -> &T { 
        &self.y 
    }
}


impl Point1Typed<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    let p = Point1Typed { x: 5, y: 10 };

    println!("p.x = {} and p.y = {}", p.x(), p.y());
}
