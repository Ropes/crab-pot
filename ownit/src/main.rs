fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    make_copy(x);

    println!("x in main: {}", x);
    // println!("s in main: {}", s); <- Compiler error; ownership moved to

    let mut s = String::from("hellow");
    change_string(&mut s);
    println!("{}", s);
    change_string(&mut s);
    println!("{}", s);

    mutable_string();
    let dangle = dangle_ptr();
    println!("{}", dangle);
    println!("first word: {}", first_word(&dangle));
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    return &s[..s.len()];
}

fn dangle_ptr() -> String{ // if this were &String, the reference would be dropped 
    let s = String::from("hellow dangle data");
    return s;
} // s goes out of its scope, it's memory allocation is dropped

fn mutable_string(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s; //error; Cannot create a mutable reference after immutable references
    let r3 = &s;

    println!("{}, {}, and {}", r1, r2, r3);
}

fn change_string(s: &mut String) {
    s.push_str(" world!")
}

fn takes_ownership(s: String) {
    println!("{}", s)
}

fn make_copy(i: i32) {
    println!("{}", i)
}

fn takes_and_gives_back(s: String) -> String {
    s
}
