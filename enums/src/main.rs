fn main() { 
 
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {width: 30, height:50, };
    println!("The {:#?} has an area of {}.", rect1, rect1.area() );

    
    let some_u8_value = Some(3u8);

    match some_u8_value {
        Some(3) => println!("three"),
        Some(n) => println!("{}", n),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("THREE");
    }

    if Some(3) == some_u8_value {
        println!("TTHHRREE");
    }
}
