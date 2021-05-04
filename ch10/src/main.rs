
use ch10::{self, Summary};

fn main(){
    let tweet = ch10::Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

fn main2() {
    let number_list = vec![34, 50, 24, 100, 65];
    let r = largest(&number_list);
    println!("largest number: {}", r);

    let number_list = vec![102, 34, 6000, 89,234,62];
    let char_list = vec!['y', 'b', 'd', 'x'];
    let r = largest(&char_list); 
    println!("largest char: {}", r);

    let integer = Point{x: 5, y: 10};
    let float = Point{x:1.0, y: 3.0};
    let mutant = Point{x: 3, y: 5.0};
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter(){
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        return &self.x;
    }
    fn y(&self) -> &U{
        return &self.y;
    }

    /* Using Point<f32>
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    */
}
