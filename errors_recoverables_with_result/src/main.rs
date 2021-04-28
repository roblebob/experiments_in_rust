use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;    
    Ok(())
}


fn get_file_handle() -> File {
    match File::open("hello.txt") {
        Ok(file) => file,        
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file:  {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file:  {:?}", other_error)
            }
        },
    }
}

fn get_file_handle2() -> File {
    File::open("hello.txt").unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {
            
            File::create("hello.txt").unwrap_or_else( |error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    })
}



fn read_username_from_file() -> Result<String, io::Error> {

    let f = File::open("hello.txt"); 
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
