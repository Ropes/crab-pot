#![allow(unused)]

// Source: https://doc.rust-lang.org/std/option/ example
// Modified to also find the largest plant
fn main() {
enum Kingdom { Plant(u32, &'static str), Animal(u32, &'static str) }

// A list of data to search through.
let all_the_big_things = [
    Kingdom::Plant(320, "douglas fir"),
    Kingdom::Plant(250, "redwood"),
    Kingdom::Plant(230, "noble fir"),
    Kingdom::Plant(229, "sugar pine"),
    Kingdom::Animal(25, "blue whale"),
    Kingdom::Animal(19, "fin whale"),
    Kingdom::Animal(15, "north pacific right whale"),
];

// We're going to search for the name of the biggest animal,
// but to start with we've just got `None`.
let mut name_of_biggest_animal = None;
let mut size_of_biggest_animal = 0;
let mut name_of_biggest_plant = None;
let mut size_of_biggest_plant = 0;
for big_thing in &all_the_big_things {
    match *big_thing {
        Kingdom::Animal(size, name) if size > size_of_biggest_animal => {
            // Now we've found the name of some big animal
            size_of_biggest_animal = size;
            name_of_biggest_animal = Some(name);
        }
        Kingdom::Plant(size, name) if size > size_of_biggest_plant => {
            size_of_biggest_plant = size;
            name_of_biggest_plant = Some(name);
        }
        Kingdom::Animal(..) | Kingdom::Plant(..) => ()
    }
}

match name_of_biggest_animal {
    Some(name) => println!("the biggest animal is {}", name),
    None => println!("there are no animals :("),
}

match name_of_biggest_plant{
    Some(name) => println!("the biggest plant is: {}", name),
    None => println!("no big plants :("),
}
}