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

    let mut count = 0; // mutable variable for the loop

    // the value 'returned' from the break will be stored in result
    let res = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
    
    println!("result value: {}", res);

    let mut while_count = 5;

    while while_count > 0 {
        while_count -= 1;
    }

    let a = [1, 2, 3, 4, 6];

    for element in a {
        println!("element: {}", element);
    }
    
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
