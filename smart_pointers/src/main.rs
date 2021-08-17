#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    assert_eq!(format!("{:?}", list), "Cons(1, Cons(2, Cons(3, Nil)))");

    
    let x = 5;
    let y1 = &x;
    let y2 = Box::new(x);
    let y3 = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y1);    // deref a reference pointing to the value of x 
    assert_eq!(5, *y2);    // deref an instance of a box pointing to a copied value of x
    assert_eq!(5, *y3);

}

// Note: this version will not store its data on the heap; focus: it's pointer like behavior 
struct MyBox<T>(T);  // tupel struct with one element of type T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
