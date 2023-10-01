fn fib (second_last: i32, last: i32, more: i32) -> i32 {
    let sum = second_last + last;
    if more <= 0 {
        return sum
    }
    fib(last, sum, more - 1)
}

fn main () {
    println!("Result: {}", fib(1, 2, 6))
}
