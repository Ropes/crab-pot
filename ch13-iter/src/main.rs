use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("slow calculation...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("today, do {} pushups!", expensive_closure(intensity));
    }
}
