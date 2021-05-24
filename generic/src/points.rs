pub mod one_type {
    
    pub struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {

        pub fn x(&self) -> &T {
            &self.x
        }

        pub fn create(x_in: T, y_in: T) -> Point<T> {    
            Point { 
                x: x_in,
                y: y_in,
            }
        }
    }
    
    impl Point<f32> {
        pub fn distance_from_origin(&self) -> f32 {
            
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
}



pub mod two_types {

    pub struct Point<T, U> {
        pub x: T,
        pub y: U,
    }

    impl<T, U> Point<T, U> {
        pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x ,
                y: other.y ,
            }
        }
    }

}



