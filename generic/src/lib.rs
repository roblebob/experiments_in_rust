mod generic_points {

    pub mod point_1typed {
        
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
    }



    pub mod point_2typed {

        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x ,
                    y: other.y ,
                }
            }
        }

    }
}




pub fn test_1typed() {

    let p = crate::generic_points::point_1typed::Point::create(5, 10 );

    println!("p.x = {}", p.x());
}
