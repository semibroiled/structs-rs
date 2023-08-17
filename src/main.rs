#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");

    let rect1: Rectangle = Rectangle {
        width: 50,
        height: 30,
    };

    println!(" The area is {} square meters", area(&rect1)); //call with rect1 reference
    println!("{:#?}", rect1)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
