use std::process;
use std::env;
use ch12::*;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config){
        eprintln!("application err: {}", e);
        process::exit(1);
    }
}
