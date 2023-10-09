fn fizz_buzz(num: i32) -> String {
    let mut result = String::from("");
    if num % 3 == 0 {
        result += "Fizz "
    }
    if num % 5 == 0 {
        result += "Buzz "
    }
    if result == "" {
        result += &format!("{} ", num);
    }
    result
}


fn main () {
    for num in 1..100 {
        println!("{}", fizz_buzz(num));
    }
}
