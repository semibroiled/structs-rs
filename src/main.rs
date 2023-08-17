#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn fit_in(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height < other.height
    }
}
fn main() {
    println!("Hello, world!");

    let rect1: Rectangle = Rectangle {
        width: dbg!(5 * 10),
        height: 30,
    };

    let rect2 = Rectangle::square(5);

    println!(" The area is {} square meters", rect1.area(&)); //call with rect1 reference
    println!("{:#?}", rect1);

    println!("Rect 2 can fit in Rect 1 is {}", rect1.fit_in(&rect2));
}
