fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string '{:?}'", i);
        },
        _ => {},
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The 'if let' construct reads: "if 'let' destructures 'number' into
    // 'Some(i)', evaluate the block ('{}').
    if let Some(i) = number {
        println!(" Matched {:?}!",i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        // Destructure failed. Change to hte failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.

    if let Some(i) = emoticon {
        println!("Matched {:?}!",i);
    // Destructure failed. Evaluate an 'else if' condition to see if the
    // alternate failure branch should be taken:
    } else if let Some(i) = letter {
        println!("Matched {:?}",i);
    } else {
        // The condition evaluated to false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon");
    }

    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }    

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this wil print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
