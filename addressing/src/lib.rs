#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
pub enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub enum Message {
    Quit,                       // No data
    Move { s: i32, v: i32 },    // Struct with data inside
    Write(String),              // void method writes data
    ChangeColor(i32, i32, i32), // mutating method which takes new color parameters
}

impl Message {
    fn call(&self) {
        println!("message calling home")
    }
}

//let m = Message::Write(String::from("hihi"));
// m.call();

#[derive(Debug)]
pub enum UsState {
    Alaska,
    Idaho,
    Oregon,
    Washington,
}

#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}
