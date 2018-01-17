use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // why Self?
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
    
fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Using the Into trait will typically require specification of the type to convert into as the
    // compiler is unable to determine this most of the time.
    // Try removing the type
    let num2: Number = int.into();
    println!("My into number is {:?}", num2);
}
