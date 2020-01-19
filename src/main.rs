// example program using structs
// ch05-02-example-structs

// debug trait! quick and dirty so we can use "{:?}" in println macro
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// move area fn to a dot-method for the Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
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

    println!("the are of the rectangle is {}", rect1.area());
    println!("{:?}", rect1);
}
