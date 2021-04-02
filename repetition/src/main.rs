fn loopin() -> i32 {
    let mut cnt = 0;
    let result = loop {
        cnt += 1;

        println!("loopin {}", cnt);
        if cnt >= 10 {
            break cnt + 2;
        }
    };
    result
}

fn whilein() {
    let mut num = 3;
    while num > 0 {
        println!("{}", num);
        num -= 1;
    }
    println!("liftoff!");
}

fn iter_arr(){
    let a = [10, 20, 30, 40, 50];
    for element in a.iter(){
        println!("iter: {}", element)
    }
}

fn iter_takeoff(){
    for num in (1..4).rev() {
        println!("{}", num);
    }
    println!("liftoff!!!")
}

fn fib_iter(n: i32) -> u128{
    let mut x = 0;
    let mut y = 1;
    let mut i = 0;

    while i < n {
        let t = y;
        y = x + y;
        x = t;
        i+=1;
    }
    x + y // return expression
}

fn main() {
    println!("Hello, world!");
    println!("{}", loopin());
    whilein();
    iter_arr();
    iter_takeoff();

    let fib = fib_iter(100);
    println!("fib: {}", fib);
}


