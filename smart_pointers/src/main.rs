fn main() {

    {
        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        use List::{Cons, Nil};
        
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        
        assert_eq!(format!("{:?}", list), "Cons(1, Cons(2, Cons(3, Nil)))");
    }





    {
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
    }




    {
        struct CustomSmartPointer {
            data: String,
        }

        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data `{}`!", self.data);
            }
        }


        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointer created.");
    }






    {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};
        use std::rc::Rc;

        let a = Rc::new( Cons(5, Rc::new( Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));

        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));

        {
            let c = Cons(4, Rc::clone(&a)); 
            println!("count after creating c = {}", Rc::strong_count(&a));
        }

        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }


}





