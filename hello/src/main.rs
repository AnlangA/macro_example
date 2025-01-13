#[macro_use]
use pro::Hello;

#[derive(Hello)]
struct Example;

impl Example {
    fn hi(&self) {
        println!("Hi");
    }
}

fn main() {
    let e = Example {};
    e.hello_world();
    e.hi();
}
