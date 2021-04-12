use addressing::{Coin, value_in_cents}; //,value_in_cents(coin);
use addressing::IpAddrKind;
use addressing::UsState;

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("{:?} {:?}", home, loopback);

    let coins = [
        Coin::Quarter(UsState::Alaska),
        Coin::Quarter(UsState::Oregon),
        Coin::Dime,
    ];

    for v in coins.iter() {
        let c = v;
        println!("coin: {:?}: {}", v, value_in_cents(c));
    }
}
