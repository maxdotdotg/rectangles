// example program using structs
// ch05-02-example-structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // first, variables are separate
    //let width1 = 30;
    //let height1 = 50;

    // refactored to be elements of one variable
    // let rect1 = (30, 50);

    // refactored to be an instance of Rectangle
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("the are of the rectangle is {}", area(&rect1));
    println!("{:?}", rect1);

}

// refactored to take a reference to Rectangle as input
// previously took dimensions as ints or as a tuple
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
