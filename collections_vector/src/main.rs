fn main() {

    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);


    let v = vec![1, 2, 3];
    println!("{:?}", v);

    {
        let mut v = Vec::new();
        v.push(4);
        v.push(5);
        println!("{:?}", v);
    }

    println!("{:?}", v);


    let third: &i32 = &v[2];         // a reference to the third element
    println!(".3rd:  {:?}", third);
    match v.get(2) {                // gives a Option<&i32>               

        Some(third) => println!("..3rd:  {:?}", third),
        None => println!("..3rd:  NONE"),
    }


    let i = 100;
    // let does_not_exist = &v[i];   // will cause panik!
    let does_not_exist = v.get(i);
    println!("does not exists:   {:?}", does_not_exist);




    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = v;
    for i in &mut v {
        *i += 50;     // derefence operator
    }
    println!("{:?}", v);

    
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let v = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", v);


}
