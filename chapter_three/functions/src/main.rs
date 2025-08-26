fn main() {
    println!("Hello, world!");
    
    another_function(five());
}

// type of every parameter must be declared
fn another_function(x: i32) {
    println!("parameter value of x is: {}", x);
}

fn five() -> i32 {
    5
}

// rust is an expression based language
