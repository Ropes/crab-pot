#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn werk() {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        // Borrowing rules vector from being mutated after declaration.
        //let pushed = 6;
        //v.push(pushed);

        println!("The first element is: {}", first);
        assert_eq!(v.len(), 5);
    }
}

fn iterating() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

fn iterating_mutable() {
    let mut v = vec![100, 23, 69];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn colating() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    let mut v_macro = vec![1, 2, 3, 4, 5];

    // add data!
    v.push(4);

    let third = &v_macro[2];
    println!("Third element: {}", third);

    match v_macro.get(2) {
        Some(third) => println!("there is a third element: {}", third),
        None => println!("no third element"),
    }

    let does_not_exist = &v[100]; // causes panic
    let also_not_exist = v.get(100);
}

fn strings() {
    let data = "initial contents";
    let s = data.to_string();
    let hello = String::from("konnichiwa");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

fn string_appending() {
    let mut s = String::from("lo");
    s.push('l');

    {
        let s1 = String::from("hellow");
        let s2 = String::from("orld!");
        let s3 = s1 + &s2; // s1 has been moved here, cannot be used again
        println!("s3: {}", s3);
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
    }
}

use std::collections::HashMap;

pub fn get_hashty(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key,value);
    }

    println!("{:?}", scores);
}

pub fn hash_updates(){
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
