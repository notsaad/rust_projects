fn main() {
    let number: i32 = 5;
    
    // rust will not convert types though
    // so doing 'if number' would not compile (unlike javascript or ruby)
    if number < 5 {
        println!("number is less than five");
    } else if number > 5 {
        println!("number is greater than five");
    } else {
        println!("number is five!");
    }
    
    // both results need to be of the same type
    let conditional_number: i32 = if true { 13 } else { 8 };
    println!("This is the conditional number: {}", conditional_number);
}
