fn main() {
    // rust programs are (mostly) made up of a series of statements
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    let x = 5u32;
    
    // blocks are expressions too
    let y =  {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to 'y'
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon supresses theis expression and '()' is assigned to 'z'
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
