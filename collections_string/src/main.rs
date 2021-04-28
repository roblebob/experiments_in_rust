fn main() {

    // let mut s = String::from("foo");
    let mut s1 = "foo".to_string();
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}  {}", s1, s2);


    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;       // s1 has been moved here and can no longer be used
    println!("{}  {}", s2, s3);


    //let hello = "Здравствуйте"; 
    let hello = "hello";
    let s = &hello[0..1];
    println!("{}  {} ", hello, s);

    
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }


	for b in "नमस्ते".bytes() {
		println!("{}", b);
	}
}
