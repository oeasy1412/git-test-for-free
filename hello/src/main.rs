fn return_value<T>(value: T) -> T {
    value
}

fn main() {
    let a=10000000;
    let b = return_value(20 * 10);
    println!("i32: {}", b);
    println!("Hello, world!");
}