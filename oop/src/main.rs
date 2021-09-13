fn main() {
    
    {
        use oop::gui::Draw;

        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }

        impl Draw for SelectBox {
            fn draw(&self) {
                // code to actually draw a select box
            }
        }


        use oop::gui::{Button, Screen};
     
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height:10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();
    }

//    {
//        use oop::blog::Post;
//
//        let mut post = Post::new();
//
//        post.add_text("I ate a salad for lunch today");
//        assert_eq!("", post.content());
//
//        post.request_review();
//        assert_eq!("", post.content());
//
//        post.approve();
//        assert_eq!("I ate a salad for lunch today", post.content());
//    }
}
