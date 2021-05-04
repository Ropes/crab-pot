
fn main(){
    let number_list = vec![34, 50, 24, 100, 65];
    let r = largest(&number_list);
    println!("largest number: {}", r);

    let number_list = vec![102, 34, 6000, 89,234,62];
    let char_list = vec!['y', 'b', 'd', 'x'];
    let r = largest(&char_list); 
    println!("largest char: {}", r);

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
