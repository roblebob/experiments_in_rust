fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);


    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue:  {:?}", score);


    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    // updating, overwriting
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // updating, only if entry does not exists, yet
    println!("{:?}", scores.entry(String::from("Yellow")));
    println!("{:?}", scores.entry(String::from("Red")));
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(100);
    println!("{:?}", scores);




    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}    {:?}", text, map);
}
