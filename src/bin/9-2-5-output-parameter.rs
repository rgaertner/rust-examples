fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut(suffix: String) -> Box<FnMut(String)> {
    let mut text = "FnMut".to_owned();
    text.push_str(suffix.as_str());
    Box::new(move |x: String| println!("This is a: {} {}",x, text))
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut(String::from(" modified!"));

    fn_plain();
    fn_plain();
    fn_mut(String::from("extra"));
    fn_mut(String::from("super"));
}
