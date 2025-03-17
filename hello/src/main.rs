fn return_value<T>(value: T) -> T {
    value
}

fn main() {
    let b = return_value(20 * 25);
    println!("i32: {}", b);
    println!("Hello, world!");
}