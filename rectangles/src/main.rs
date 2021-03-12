fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "area_simple(width1, height1) = {}",
        area_simple(width1, height1)
    );

    let tuple = (30, 50);
    println!(
        "area_tuples(tuple) = {}",
        area_tuples(tuple)
    );

    let rectangle1 = Rectangle { 
        width: 30, 
        height: 50 
    };
    println!("rectangle1 is {:?}", rectangle1);
    println!("rectangle1 is {:#?}", rectangle1);
    println!(
        "area_struct(rectangle1) = {}",
        area_struct(rectangle1)
    );
    println!(
        "rectangle1.area() = {}",
        rectangle1.area()
    );


    let rectangle2 = Rectangle { 
        width: 30, 
        height: 50 
    };
    println!(
        "rectangle2.can_hold(&rectangle1) = {}",
        rectangle2.can_hold(&rectangle1)
    );

    let square = Rectangle::square(3);
    println!(
        "Square {:?}", square
    );
}

fn area_simple(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area_struct(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}