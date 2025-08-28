fn main() {
    println!("Hello, world!");
    
    another_function(five());
}

// type of every parameter must be declared
fn another_function(x: i32) {
    println!("parameter value of x is: {}", x);
}

fn five() -> i32 {
    // 5 ALLOWED
    // 4+1 ALLOWED
    // 4 + 1; NOT ALLOWED bc its a statement which doesnt evaluate to a value
    4 + 1
}

// rust is an expression based language
