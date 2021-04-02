
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    
    let x = 5;
    make_copy(x);

    println!("x in main: {}", x);
}

fn takes_ownership(s: String){
    println!("{}",s)
}

fn make_copy(i: i32){
    println!("{}", i)
}

fn takes_and_gives_back(s: String) -> String {
    s
}