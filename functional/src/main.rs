use std::thread;
use std::time::Duration;
use functional::cacher::Cacher;


fn main() {
    // workout();
    


    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));




    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}





fn workout() {

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new( |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(&intensity));
        }
    }
}

