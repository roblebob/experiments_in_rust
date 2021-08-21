#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    assert_eq!(format!("{:?}", list), "Cons(1, Cons(2, Cons(3, Nil)))");


    use smart_pointers::the_deref_trait::MyBox;    

    let x = 5;
    let y1 = &x;
    let y2 = Box::new(x);
    let y3 = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y1);    // deref a reference pointing to the value of x 
    assert_eq!(5, *y2);    // deref an instance of a box pointing to a copied value of x
    assert_eq!(5, *y3);


    let m = MyBox::new(String::from("Rust"));

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    hello(&m);  // thank's to deref coersions due to the Deref implementation in MyBox

    hello(&(*m)[..]);  // ... without




    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
}



struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


