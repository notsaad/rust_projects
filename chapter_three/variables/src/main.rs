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
    println!("The value of y is: {}", Y);
}
