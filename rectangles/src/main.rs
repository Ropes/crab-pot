
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn square(size: u32)-> Rectangle{
        return Rectangle{
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32{
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("width: {} height: {}", width1, height1);

    let rect1 = (30, 50);
    println!("area: {}", area1(rect1));

    let rect2 = Rectangle{width: 40, height: 50};
    println!("area: {}: rect: {:?}", area(&rect2), rect2);
    println!("area: {}: rect: {:?}", rect2.area(), rect2);

    let rect3 = Rectangle{width:10, height:10};
    let rect4 = Rectangle{width:100, height: 10};
    let rect5 = Rectangle::square(30);
    println!("rect {:?} can hold: {:?}: {}?",rect3, rect4, rect3.can_hold(&rect4));
    println!("rect {:?} can hold: {:?}: {}?",rect5, rect3, rect5.can_hold(&rect3));


}

fn area1(dimensions:(u32, u32)) -> u32{
    return dimensions.0 * dimensions.1;
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height
}

