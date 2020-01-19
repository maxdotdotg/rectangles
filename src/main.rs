// example program using structs
// ch05-02-example-structs

// debug trait! quick and dirty so we can use "{:?}" in println macro
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// move area fn to a dot-method for the Rectangle struct
// self is implicitly the type being implemented
// makes sense, we're extending the type, I think?
impl Rectangle {
    // uses self, dot-method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // uses self, dot-metod
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // does NOT use self, associated function (not a method?)
    // nope, not a method
    // "They’re still functions, not methods, because they don’t have an 
    // instance of the struct to work with"
    // ch05-03-method-syntax
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // first, variables are separate
    //let width1 = 30;
    //let height1 = 50;

    // refactored to be elements of one variable
    // let rect1 = (30, 50);

    // refactored to be an instance of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the are of rect1 is {}", rect1.area());
    println!("{:?}", rect1);

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(4);
    println!("sq is: {:?}", sq);
}
