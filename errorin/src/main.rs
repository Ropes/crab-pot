
use std::io;
use std::{fs::File, io::Read, io::ErrorKind};
use std::net::IpAddr;


fn main(){
    let v = vec![1,2,30];
    let p = "hello.txt";

    hellotxt(p);

    hello_shortcut(p);

    let r = read_from_file(p);
    let o = r.expect("error reading from file");
    println!("file contents: {}", o);

    let r = read_username_from_file(p);
    println!("streamlined file contents: {}", r.expect("error reading from file"));
}


fn hellotxt(path: &str){
    let f = File::open(path);
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path){
                Ok(fc) => fc,
                Err(e) => panic!("problem creating file: {:?}", e),
            },
            other_error => panic!("problem opening file: {:?}", other_error),
        },
    };
}

fn hellofile(path: &str){
    let f = File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound{
            File::create(path).unwrap_or_else(|error| {
                panic!("error creating file: {:?}", error);
            })
        }else {
            panic!("unrecoverable error opening file: {:?}", error);
        }
    });
}

fn hello_shortcut(path: &str){
    let _f = File::open(path).expect("failed to open file");
}


fn read_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(path)?.read_to_string(&mut s)?;

    return Ok(s);
}

fn parse_ipv4(addr: &str) -> Result<IpAddr, io::Error> {
    //                           unwrap() -> Panics on failure; discouraged since 'addr' is unknown
    let p: IpAddr = addr.parse().unwrap();
    return Ok(p);
}