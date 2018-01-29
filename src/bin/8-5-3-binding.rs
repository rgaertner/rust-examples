// a function 'age' which returns 'u32'
fn age() -> u32 {
    26
}

fn main() {
    println!("Tell me what type of person you are");

    match age(){
        0 => println!("I am not born yet I guess!"),
        // Could match '1 ... 12' directly but then what age 
        // would the child be? Instead, ind to 'n' for the 
        // sequence of 1 ... 12. Now the age can be reported.
        n@ 1 ... 12 => println!("I'm a child of age {:?}", n),
        n@ 13 ... 19 => println!("I'm a teen of age {:?}", n),
        n@ 21 ... 23 | n@ 25 ... 27 => println!("I'm a twen of age {:?} and not 24", n),
        n => println!("I'm an old person of age {:?}",n ),
    }

    let name = "Bob";

    match name {
        n@ "Bob" | n@ "Frank" | n@ "Ferris" => println!("found {} in names", n), n => println!("Dunno {}", n),
    }

    let pair = (1,2);

    match pair {
        n @(1,2) => println!("success"),
        _ => println!("failure"),
    }
}
