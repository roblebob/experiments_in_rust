fn main() {

    {
        use generic::points::one_type::Point;

        let p = Point::create(5, 10);

        println!("p.x = {}", p.x());
    }

    {
        use generic::points::two_types::Point;
    
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);

        println!("p3.x = {}  p3.y = {}", p3.x, p3.y);
    }




    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let string_list = vec!["hallo", "hillalo", "hulo", "xylon", "sgugug"];

    {
        use generic::largest::copy::largest;    
        println!("The largest number is: {}", largest( &number_list));
        println!("The largest char is: {}", largest( &char_list));
    }

    {
        use generic::largest::clone::largest;    
        println!("The largest string is: {}", largest( &string_list));
    }

    {
        use generic::largest::reference::largest;
        println!("The largest string 2: {}", largest( &string_list));
    }










    use generic::media_aggregator::notify;
    use generic::media_aggregator::NewsArticle;
    use generic::media_aggregator::Tweet;
    use generic::media_aggregator::Summary;





    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());



    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };

    println!("I new tweet: {}", tweet.summarize());



    notify( &tweet);







}
