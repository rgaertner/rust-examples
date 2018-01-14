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
This just  works if you set the bin folder of the cargo installation to your $PATH.
More details can be found in the book [The Rust Programming Language](https://doc.rust-lang.org/book/second-edition/ch14-04-installing-binaries.html) 

```sh
git submodule init
cargo install mdbook
mdbook serve rust-by-example # host rust-by-example on localhost:3000
```





