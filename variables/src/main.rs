fn accept_tup(t: (i32, f64, i32)) {
    let (_x, y, _z) = t;
    println!("Hello, world! {} {}", t.0, y);
}

fn expression_v_statement() {
    let x = 5;
    let y = {
        let x = 3; // statement
        plus_one(x) // function returning expression
    }; // y = 4
    println!("X: {}, Y: {}", x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // expression
}

fn main() {
    let tup = (500, 6.4, 1);

    accept_tup(tup);
    expression_v_statement();
}
