fn largest_i32( list: &[i32]) -> i32 {

    let mut largest = list[0];

    for &item in list {   if item > largest { largest = item; }   }
    
    largest
}

fn largest_char( list: &[char]) -> char {

    let mut largest = list[0];

    for &item in list  {   if item > largest { largest = item; }   }

    largest
}

fn largest_with_copy<T: PartialOrd + Copy >( list: &[T]) -> T {

    let mut largest = list[0];

    for &item in list {   if item > largest { largest = item; }   }

    largest
}








fn main() {

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_with_copy( &number_list);
    println!("The largest number is: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_with_copy( &char_list);
    println!("The largest char is: {}", result);




    let p = Point1Typed { x: 5, y: 10 };
    println!("p.x = {} and p.y = {}", p.x(), p.y());

    
    let p1 = Point2Typed { x: 5, y: 10.4 };
    let p2 = Point2Typed { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}  p3.y = {}", p3.x, p3.y);
}



struct Point1Typed<T> { x: T, y: T, }

impl<T> Point1Typed<T> {
    fn x(&self) -> &T { &self.x }
    fn y(&self) -> &T { &self.y }
}

impl Point1Typed<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}



struct Point2Typed<T,U> { x: T, y: U, }

impl<T, U> Point2Typed<T, U> {
    fn mixup<V, W>(self, other: Point2Typed<V, W> ) -> Point2Typed<T, W> {
        Point2Typed{
            x: self.x,
            y: other.y,
        }
    }
}






