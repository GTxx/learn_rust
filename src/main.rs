mod fib;
use fib::fib;

fn main() {
    println!("Hello, world!");
    let x = "hello, world1";
    println!("{}", x);
    let y: i32 = 123;
    hello::print_hello();
    let mut z = fib(2);
    println!("{}", z);
    z = fib(12);
    println!("{}", z);
}

mod hello {
    pub fn print_hello(){
        println!("hello, world2");
    }
}