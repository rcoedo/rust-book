#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    let r2 = Rectangle {
        width: 20,
        height: 30,
    };
    let r3 = Rectangle {
        width: 20,
        height: 60,
    };
    let s1 = Rectangle::square(10);

    println!("The area of the rectangle is {} square pixels.", r1.area());
    println!("The area of the square is {} square pixels.", s1.area());

    println!("Can r1 hold r2? {}.", r1.can_hold(&r2));
    println!("Can r1 hold r3? {}.", r1.can_hold(&r3));
    println!("Can r2 hold r3? {}.", r2.can_hold(&r3));

    println!("rectangle is {:?}", r1);
}
