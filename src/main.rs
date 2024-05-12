/*
Generics in Rust are a powerful feature that allows you to define functions, structs, enums, and traits with placeholders 
for data types. These placeholders can be used to create flexible and reusable code by abstracting over different types, 
enabling you to write code that operates on a wide range of data types without sacrificing type safety.

Generics in Rust provide a way to write functions or data structures that can work with any data type, rather than being tied 
to a specific type. This makes your code more flexible and adaptable, as it can be used with different types without having to 
rewrite or duplicate code.

Generics in Rust are similar to templates in other languages like C++ or Java. They allow you to define code that operates on 
unspecified types, which are then filled in at compile time when the code is used with specific types.
*/


struct Point{    // simple struct
    x: i32,
    y: i32,
}

struct Location<T,U>{
    x: T,
    y: U,
}

impl<T,U> Location<T,U> {
    fn new(x: T, y: U) -> Location<T,U> {
        Location {x, y}
    }
}
impl Location<i32, i32> {
    fn printing(&self){
        println!("the value og the coordinates are {},{}", self.x, self.y);
    }
}

fn add_points<T,U>(p1: &Location<T,U>, p2: &Location<T,U>) -> Location<T,U>{
    unimplemented!();
}

fn main() {
    let origin = Point {x: 0, y: 1};
    //let p1 = Point {x: 1.0, y: 2.0};     
    /* we get an error of mismatched types as an i32 was wxpected and a floating point was incountered
    the generics comes in to play in such satuations 
    to allow the poin to support a variety of concrete types we can use the generics */

    let l1: Location<i32, i32> = Location::new(0, 0);
    let l2: Location<f32, f32> = Location::new(1.0, 2.0);
    let L3: Location<i32, f32> = Location::new(21, 22.0);
    l1.printing();
}
