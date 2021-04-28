fn main() {
    
    let mut x:u8 = 250;

    x = plus_one(x);

    println!("The value of x is {}", x);
}




fn plus_one(x: u8) -> u8 {

    match x.overflowing_add(1) {
        (_y,false) => _y,
        (_y,true) => 255
    }
}
