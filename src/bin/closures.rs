use std::{thread, time::Duration};

// A closure is a like a function without any name
// can be stored in a variable and passed as a parameter

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    print!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main(){
    let simulated_intensity = 10;
    let simulated_random_number = 7;
    
    generate_workout(simulated_intensity, simulated_random_number);
}

struct Cacher <T> // Using generic called T
where
    T: Fn(u32) -> u32, // Using trait Fn
{
    calculation: T,
    value: Option<u32>,
}

fn generate_workout (intensity: u32, random_number: u32) {
    // Here we are calling simuated fn every time even though it's not need at random_number == 3
    // let amount = simulated_expensive_calculation(intensity);
    // So we use closure(it's like function inside another function)
    // We don't need to enter the type of input and return values
    // It decides the type from the 1st line where it is used
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // To solve the double below we use struct Cacher

    if intensity < 25{
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Today, do {} situps!",
            expensive_closure(intensity)
        );
    }
    else {
        if random_number == 3{
            println!("Take a break Today!! Remember to stay hydrated");
        }
        else {
            println!(
                "Today run for {} minutes",
                expensive_closure(intensity)
            );
        }
    }
}