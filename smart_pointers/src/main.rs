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



    println!("---------------------------------------------");


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


    println!("---------------------------------------------");


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



    println!("---------------------------------------------");



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


    println!("---------------------------------------------");


    {
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};
        use std::cell::RefCell;
        use std::rc::Rc;

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }


    println!("---------------------------------------------");


    {
        use List::{Cons, Nil};
        use std::cell::RefCell;
        use std::rc::Rc;
     
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    Nil => None,
                }
            }
        }


        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count afer b created = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncommenent the next line to see that we have a cycle; it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    

    } //The reference count of the Rc<List> instances in both a and b are 2 after we change the list in a to point to b. At the end of the scope, Rust drops the variable b, which decreases the reference count of the Rc<List> instance from 2 to 1. The memory that Rc<List> has on the heap won’t be dropped at this point, because its reference count is 1, not 0. Then Rust drops a, which decreases the reference count of the a Rc<List> instance from 2 to 1 as well. This instance's memory can’t be dropped either, because the other Rc<List> instance still refers to it. The memory allocated to the list will remain uncollected forever. 



    

    println!("---------------------------------------------");

    {
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let leaf = Rc::new(Node {
            value: 3, 
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!("leaf strong = {}, leaf weak = {}", 
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));


        println!("....");
        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
            
            println!("branch strong = {}, branch weak = {}", 
                     Rc::strong_count(&branch),
                     Rc::weak_count(&branch));

            println!("leaf strong = {}, leaf weak = {}", 
                     Rc::strong_count(&leaf),
                     Rc::weak_count(&leaf));
        

        }
        println!("....");

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!("leaf strong = {}, leaf weak = {}", 
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));
    }
}





