// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// Create an enum to classify a web event. Note how both 
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and 
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `b` from inside the `enum`.
        WebEvent::KeyPress(b) => println!("pressed '{}'.", b),
        WebEvent::Paste(w) => println!("pasted '{}'.", w),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click {x: 20, y: 80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

}
