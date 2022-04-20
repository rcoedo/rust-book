fn main() {
    for i in 1..10 {
        println!("fib({}) = {}", i, fib(i));
    }
}

fn fib(x: u32) -> u32 {
    match x {
        0 => 0,
        1 => 1,
        _ => fib(x - 2) + fib(x - 1),
    }
}
