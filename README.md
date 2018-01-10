# About

Example code produced while learning Rust from [rustbyexample](http://rustbyexample.com/) -> [github ](https://github.com/rust-lang/rust-by-example)

# Prerequisites

[rust in version >= 1.22](https://www.rust-lang.org/en-US/install.html) 

# Setup 
```sh
git clone https://github.com:rgaertner/rust-examples.git
cd rust-examples
```
## run examples
```sh
cargo run --bin [name] # find available names via ls src/bin
```
## read the book locally
```sh
git submodule init
cargo install mdbook
mdbook serve rust-by-example # host rust-by-example on localhost:3000
```





