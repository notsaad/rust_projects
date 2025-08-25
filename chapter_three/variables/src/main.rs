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
}
