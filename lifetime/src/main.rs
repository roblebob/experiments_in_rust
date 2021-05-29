

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }
}



fn main() {
    
    let string1 = String::from("abcdefg");
    let result;
    let string2: String;
    {
        string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);




    let novel = String::from("Call me Ishmael. Some years ago....");
    let first_sentence = novel.split('.').next().expect("Coul not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);





    let s: &'static str = "I have a static lifetime.";

}




struct ImportantExcerpt<'a> {

    part: &'a str,
}


impl<'a> ImportantExcerpt<'a> {

    fn level(&self) -> i32 {
        3
    }

    fn announcement_and_return_part(&self, announcement: &str) -> &str {
        println!("Attendion please: {}", announcement);
        self.part
    }
}







use std::fmt::Display;

fn longest_with_an_announcement<'a, T> (
    x: &'a str, 
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Annoucement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
