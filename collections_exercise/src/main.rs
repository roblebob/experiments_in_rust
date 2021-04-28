use rand::Rng;
use std::collections::HashMap;
use std::io;


fn main() {
    
    // Exercise 1
    let vec = vector();    
    println!("{:?}", vec);
    println!("length:  {}", vec.len());
    println!("average: {}", average(&vec));
    println!("median:  {}", median(&vec));
    println!("mode:    {}", mode(&vec));

    // Ecercise 2
    let text = "hello world wonderful world with Äpfel".to_string();
    println!("{}", text);
    println!("{}", to_pig_latin(&text));

    // Exercise 3
    add_employee_to_department();
}


fn vector() -> Vec<i32> {
    let size = rand::thread_rng().gen_range(10, 100);
    let mut v = Vec::new();
    for _i in 0..size { v.push(rand::thread_rng().gen_range(0, size));}
    v
}

fn average(vec: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    for i in vec { sum += *i;}
    (sum as f32) / (vec.len() as f32)
}

fn median(vec: &Vec<i32>) -> i32 {
    let mut v = vec.clone();  
    v.sort();
    v[v.len() / 2]
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in vec {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by( |a, b| b.1.cmp(&a.1));
    v[0].0
}



fn to_pig_latin(s: &String) -> String {
    let consonants = vec!['B', 'b', 'C', 'c', 'D', 'd', 'F', 'f', 'G', 'g', 'H', 'h', 'J', 'j', 'K', 'k', 'L',  'l', 'M', 'm', 'N',  'n', 'P',  'p', 'Q',  'q', 'R',  'r', 'S',  's', 'T', 't', 'V', 'v', 'W', 'w', 'X',  'x', 'Y',  'y', 'Z', 'z'];
    let vowels = vec!['A', 'a', 'Ä', 'ä', 'E', 'e', 'I', 'i', 'O', 'o', 'Ö', 'ö', 'U', 'u', 'Ü', 'ü'];
    
    let v: Vec<_> = s.split_whitespace().collect();
    let mut v_out: Vec<String> = Vec::new();

    for i in &v {
        
        let mut word = (*i).to_string();
        let first = word.remove(0);

        if consonants.iter().any(|&c| c==first) {
            v_out.push( format!( "{}-{}ay", word, first ));        
        } else if vowels.iter().any(|&c| c==first) {
            v_out.push( format!( "{}{}-hay", first, word));        
        } else {
            v_out.push( format!( "{}{}", first, word));
        }
    }
    let mut s = v_out.into_iter().fold("".to_string(), |a, b| format!("{} {}", a, b));
    s.remove(0);
    s
}


fn add_employee_to_department() {

    let _line = "---------------------------";
    let _heading = "COMMANDS";
    let _insert_command = "Add <name> to <department>";
    let _print_command = "Print <department>";
    let _print_all_command = "Print all";
    let _quit_command = "Quit";
    let _intro = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",  _line, _heading, _line, _insert_command, _print_command, _print_all_command, _quit_command, _line );
    
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("{}", _intro);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let v: Vec<_> = input.split_whitespace().collect();

        if v.len() == 1 && v[0] == "Quit" {
            break;
        } else if v.len() == 2 && v[0] == "Print" {
            if v[1] == "all" {
                let mut names: Vec<String> = Vec::new();
                for (_, n) in &company {
                    let mut name = (*n).clone();
                    names.append(&mut name);
                }
                names.sort();
                println!("{:?}", names);
                continue;
            } 
            println!("{:?}", company.get(v[1]));
            continue;
        } if v.len() == 4 && v[0] == "Add" && v[2] == "to" {
        
            let department = company.entry(v[3].to_string()).or_insert(vec![v[1].to_string()]);
            if (*department).iter().any(|i| i==v[1]) {
            } else {
                (*department).push(v[1].to_string());
                (*department).sort();
            }
            continue;
        } 
        println!("Error!!");
    }
}





