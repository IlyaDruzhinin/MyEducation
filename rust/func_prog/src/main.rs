use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    let res = fact1(5);
    println!("factorial 5: {}\n", res);
    let res2 = sum_from_to(5, 10);
    println!("res2: {}", res2);
}

// Factorial version 1
fn fact1(x: i32) -> i32 {
    if (x == 1) {
        return 1;
    } else {
        x * fact1(x - 1)
    }
}

// Sum for all period
fn sum_from_to_v1(from: i32, to: i32) -> i32 {
    let mut res: i32 = 0;
    for num in from..to {
        res+=num
    }
    return res;
}
// в функциональном стиле
fn sum_from_to_v2(from: i32, to: i32) -> i32 {
    
}

fn generate_workout(intensity: i32, random_number: i32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity))
        }
    }
}
