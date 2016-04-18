mod fib;
use fib::fib;

mod queen;
use queen::find_queen_position; 

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
    let xx = if z == 0 {false} else {true};
    println!("{}", xx);
    match x {
        "123" => {
            println!("{}", x)
        },
        _ => println!("{}", x),
    }

    // 8 queen

}

mod hello {
    pub fn print_hello(){
        println!("hello, world2");
    }
}
