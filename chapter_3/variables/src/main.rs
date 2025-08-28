fn main() {
    let x = 5;
    const Y: u32 = 5;
    /*
    differences between immutable variable and const:
    let only lives in the scope its decalred
    const can be globally scoped
    const is conventionally all upper case
    const is always immutable
    type is required to be explicitly defined for consts
    const must be known at compile-time and is inlined wherever its used
    let is determined at runtime and stored in memory stack
    */

    println!("The value of x is: {}", x);
    // this is valid because this is shadowing, not reassigning the variable
    // shadowing means declaring a new variable with the same name as an old one
    // you can shadow a variable into a new var with a diff type too
    let x = x * 2;

    println!("The value of x is: {}", x);

    println!("The value of y is: {}", Y);
    
    // Scalar Types in Rust
    
    // integers:
    // unsigned and signed 8bit to 128bit ints
    
    // floats:
    // f32 and f64
    
    // bools 
    
    // chars: (four bytes in size and represents unicode scalar)
    // single quotes for a char vs double quotes for a string
    
    
    // Compound Types in Rust
    
    // tuples:
    // group together a number of values with a variety of types
    // fixed length, once declared cannot grow or shrink in size
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // arrays: (normal, allocated on the stack, fixed size)
    // 
    // functions are snake case with underscores
    // syntax is fn to declare function
}
