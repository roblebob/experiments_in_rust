pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }
}






use std::fmt::Display;

pub fn longest_with_an_announcement<'a, T> (
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





pub struct ImportantExcerpt<'a> {

    pub part: &'a str,
}


impl<'a> ImportantExcerpt<'a> {

    pub fn level(&self) -> i32 {
        3
    }

    pub fn announcement_and_return_part(&self, announcement: &str) -> &str {
        println!("Attendion please: {}", announcement);
        self.part
    }
}




