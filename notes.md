## __Links__

official site:    https://www.rust-lang.org/      
playground (online compiler):   https://play.rust-lang.org/   
libraries:    https://crates.io/
<br>

official book https://doc.rust-lang.org/book/    
The Rustonomicon https://doc.rust-lang.org/nomicon/



### ___extended:___

Tom's Obvious Minimal Language ___(TOML)___ https://toml.io/en/  
Semantic Versioning   ___(SemVer)___:    https://semver.org/  
Regular Expressions 101:   https://regex101.com/r/Ly7O1x/3/  




<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>



## __Rustup__: ___the Rust installer and version management tool___
<br>

- install
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
<br>

- uninstall
```shell
rustup self uninstall
```
<br>

- update
```shell
rustup update
```
<br>

- documentation
```shell
rustup doc
```


<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>



## __Cargo__:
<br>

names to be used in _snake case_

<br>

- ___generate new___ project
```shell
cargo new my_project
```
<br>


- ___generate new___ library
```shell
cargo new --lib my_library
```
<br>




- ___build___ for developing
```shell
cargo build
```
<br>

- ___run___ (`./target/debug/my_project`)
```shell
cargo run
```
<br>

- ___checking___ is faster than ___compiling___
```shell
cargo check
```
<br>

- ___build___ for release and benchmark
```shell
cargo build --release
```


<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

## Variables, mutability, and constants

### constants

* the type of the value must be annotated
* can be declared in any scope, including the global scope   
* valid for the entire time a program runs, within the scope they were declared in
* set only to a constant expression (not the result of a function call or any other value that could only be computed at runtime)
```rust
const MAX_POINTS: u32 = 100_000;
  ```

-> useful for values that many parts of code need to know about







<br><br><br>
## Types (___scalar___ & ___compound___)
- statically typed (_known at compile time_)











<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

# Ownership

+ each _value_ has a _variable_ that's called its ___owner___
+ there can only be ___one___ owner ___at a time___
+ when owner goes ___out of scope___, the value will be ___dropped___



## References & Borrowing

+ At any given time, you can have either one mutable reference or any number of immutable references.

+ References must always be valid.


## _Slice_ - the other type, that does not have ownership

### string _slices_  (`&str`)

```rust
  let s = String::from("hello world");

  let hello = &s[0..5];   // &s[..5]

  let world = &s[6..11];  // &s[6..]


```

![string slice](https://doc.rust-lang.org/book/img/trpl04-06.svg =400x)


+ [String Literals Are Slices](https://doc.rust-lang.org/book/ch04-03-slices.html#string-literals-are-slices)







<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

# Structs

## Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect1 = Rectangle {width: 30, height:50, };
println!("The {:#?} has an area of {}.", rect1, rect1.area() );

```

___first argument___

+ ```&self``` &emsp; _...reading_
+ ```&mut self``` &emsp; _...mutating_
+ ```self``` &emsp; _...consuming_

___automatic referencing & dereferencing___

given ```object.something()```
automatically adds ```&``` , ```&mut``` , or ```*```
so ```object``` matches method's signature
```Rust
p1.distance(&p2);       //  both are
(&p1).distance(&p2);    //  ... the same
```




<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

# Enums & Pattern Matching

```rust
enum Message {
    Quit,                          // has no data associated with it at all.
    Move { x: i32, y: i32 },       // includes an anonymous struct inside it.
    Write(String),                 // includes a single String.
    ChangeColor(i32, i32, i32),    // includes three i32 values.
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```


### [```Option``` Enum](https://doc.rust-lang.org/std/option/enum.Option.html)

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### The ```match``` Control Flow Operator

```Rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

+ ```match``` is enforcing exhaustive checking (_all cases must be considered to be compiled_)
  ```Rust    
  let some_u8_value = Some(0u8);

  match some_u8_value {
      Some(3) => println!("three"),
      _ => (),
  }
  ```
+ syntax sugar

<br><br><br><br>

+ difference betwwen ```if``` and ```if let```
[--->](https://doc.rust-lang.org/book/ch06-03-if-let.html#concise-control-flow-with-if-let)

<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

# The Module System

+ __Packages:__ A Cargo feature that lets you build, test, and share crates
+ __Crates:__ A tree of modules that produces a library or executable
+ __Modules__ and __use:__ Let you control the organization, scope, and privacy of paths
+ __Paths:__ A way of naming an item, such as a struct, function, or module


### Packages and Crates


+ A ___crate___ is a _binary_ or _library_.
+ A ___package___ can contain
   + at most ___one___ _library crate_
   + arbitrary ___many___ _binary crates_
   + at total, it must at least has one crate (_binary_  or _library_).















<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

# Common Collections

+ (_unlike the built-in array and tuple types,_)   
 the data these collections point to is stored on the heap,    
 &#8658; &nbsp; the amount of data does not need to be known at compile time    
 &#8658; &nbsp; can grow / shrink as the program runs

### [vector](https://doc.rust-lang.org/std/vec/struct.Vec.html)
```Rust
let v: Vec<i32> = Vec::new();   // []

let v = vec![1, 2, 3];
{
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    println!("{:?}", v);  // [4, 5]
}
println!("{:?}", v);   // [1, 2, 3]

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
enum SpreadsheetCell { Int(i32), Float(f64), Text(String), }
let v = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
println!("{:?}", v);
 ```

 ### string

```rust
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
```

 ### hash map

```rust
 use std::collections::HashMap;

 let mut scores = HashMap::new();
 scores.insert(String::from("Blue"), 10);
 scores.insert(String::from("Yellow"), 50);
 println!("{:?}", scores);

 let teams = vec![String::from("Blue"), String::from("Yellow")];
 let initial_scores = vec![10, 50];
 let mut scores: HashMap<_,_> =
     teams.into_iter().zip(initial_scores.into_iter()).collect();
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
 ```


 #### [others](https://doc.rust-lang.org/std/collections/index.html)




<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

# Error Handling



<br>

## Unrecoverable Errors with `panic`

two possibilties when `panic` occurs
+ ___unwinding___   (default)
  + Rust walks back up the stack and cleans up the data from each function it encounters
  + lots of work
+ immediately ___abort___ (alternative)
  + ends program without cleaning up
  + needs to be cleaned up by the operating system
  + if the resulting binary needs to be as small as possible


<br>

## [Recoverable Errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```




```Rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
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
    };
}
```
```Rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {

            File::create("hello.txt").unwrap_or_else( |error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

### [Propagating Errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors)

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_From_file() -> Result<String, io::Error> {

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
```

#### [A Shortcut for Propagating Errors: the `?` Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)

`?` placed after `Result` value &emsp; &#x27FF; &ensp; almost the same way as the `match` expressions above


```rust
// use ...
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
differences to `match`
+ error values that have the `?` operator called on them go through the `from`    function
  + defined in the `From` trait in the standard library,
  + used to convert errors from one type into another
  + the error type received is converted into the error type defined in the return type of the current function
  + useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
  + As long as each error type implements the `from` function to define how to convert itself to the returned error type, the `?` operator takes care of the conversion automatically
+ cases:
  + `Ok(_)` &#x27FF; returns the value inside
  + `Err(_)` &#x27FF; will return early out of the whole function + gives any `Err` value to the calling code

+ further shortening by chaining method calls immediately after the `?`
  ```Rust
  // use ...
  fn read_username_from_file() -> Result<String, io::Error> {
      let mut s = String::new();
      File::open("hello.txt")?.read_to_string(&mut s)?;
      Ok(s)
  }
  ```
+ only allowed in functions that return `Result` or `Option` or any other type implementing `std::ops::Try`

<br><br>  

## To `panic!` or Not to `panic!`

+ [Examples, Prototype Code, and Tests](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#examples-prototype-code-and-tests)

+ [Cases in Which You Have More Information Than the Compiler](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler)    
...  `unwrap()` and `expect()` methods perfectly acceptable, when an `Err` variant can be excluded
  ```Rust
  use std::net::IpAddr;
  let home: IpAddr = "127.0.0.1".parse().unwrap();
  ```
+ ___bad state___ when some _assumption_, _guarantee_, _contract_, or _invariant_  has been broken    
 &#x21B8; _invalid-_, _contradicting-_, or _missing_ values are passed &emsp;  +  ...
    + not expected to happen occasionally
    + code after this point needs to rely on not being in that state
    + encoding this information within the used types is not practicable
+







<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

# Generic Types, Traits, and Lifetimes

[___Monomorphization___](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics)  is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

<br>

## Traits

_Traits_ are similar to a feature often called ___interfaces___ in other languages, although with some differences.

Implementation of a trait for a particular type can only be done, if ether the trait itself or the type is local (___coherence___, more specifically the ___orphan rule___)


[trait-bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)


<br>

## Lifetime

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime  
```

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters
-- if not, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function.

#### [Lifetime Elision](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision)

Every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use references.     
... some can be inferred by the compiler, but only when no ambiguity


1)  each parameter that is a reference gets its own lifetime parameter.     
    ```rust
    fn foo<'a>(x: &'a i32);

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
    // ...
    ```

2)  if there is exactly one input lifetime parameter, that lifetime is assigned to all  output lifetime parameters:
    ```rust
    fn foo<'a>(x: &'a i32) -> &'a i32
    ```

3)  if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.


<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

# Test

adding the `#[derive(PartialEq, Debug)]` annotation to your `struct`  or `enum` definition, to use the `assert_eq!` and `assert_ne!` macros (_derivable traits_)


<br><br><br>

#### [Running Tests in Parallel (_default_) or Consecutively:](https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-tests-in-parallel-or-consecutively)

```shell
$ cargo test -- --test-threads=1
```

#### [Showing Function Output](https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output)

```shell
$ cargo test -- --show-output
```

#### [Running Single Tests](https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-single-tests)

```shell
$ cargo test <name-of-the-test-function>
```

#### [Filtering to Run Multiple Tests](https://doc.rust-lang.org/book/ch11-02-running-tests.html#filtering-to-run-multiple-tests)

```shell
$ cargo test <part-of-all-the-names-of-the-test-functions-to-be-tested>
```

#### [Running only the ignored tests](https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested)

```shell
$ cargo test -- --ignored
```

<br><br><br><br><br><br>

## Integration Tests

+ go in seperate folder `tests/`

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

+ [Running a particular integration test](https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-directory) (e.g.: `test/integration_test.rs`)

```shell
cargo test --test integration_test
```

+ each file is compiled as a seperated _crate_, in contrast to those in `src/`
+

<br><br><br>

[last](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
