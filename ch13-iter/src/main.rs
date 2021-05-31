use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("slow calculation...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Different closure definitions
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x: u32| x + 1;

    if intensity < 25 {
        println!("today, do {} pushups!", expensive_closure(intensity),);
        println!("next do {} situps", expensive_closure(intensity),);
    } else {
        if random_number == 3 {
            println!("take a break today")
        } else {
            println!("today run for {} minutes", expensive_closure(2),);
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
