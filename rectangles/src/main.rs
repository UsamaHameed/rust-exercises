fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_simple(width1, height1)
    );

    let tuple = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples(tuple)
    );

    let rectangle = Rectangle { 
        width: 30, 
        height: 50 
    };
    println!("rectangle is {:?}", rectangle);
    println!("rectangle is {:#?}", rectangle);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(rectangle)
    );

    let rectangle1 = Rectangle { 
        width: 30, 
        height: 50 
    };
    println!(
        "The area of the rectangle1 is {} square pixels.",
        rectangle1.area()
    );


    let rectangle2 = Rectangle { 
        width: 30, 
        height: 50 
    };
    let rectangle3 = Rectangle { 
        width: 10, 
        height: 20 
    };
    println!(
        "Can rectangle2 hold rectangle3: {}",
        rectangle2.can_hold(&rectangle3)
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

#[derive(Debug)]
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