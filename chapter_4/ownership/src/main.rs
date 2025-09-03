fn main() {
    // let s = "hello";
    let mut s = String::from("Hello");
    // let s2 = s; this would mean s2 points to the same string as s, not a copy of s
    // ^ this invalidates s, and is called moving s to s2
    // rust never automatically creates 'deep' copies of data
    // .clone() method provides a deep copy of the heap data as well (however is expensive)
    s.push_str(", world!");
    
    // for variables stored on the stack (that are known at compile time) like integers
    // there's no moving (bc there's no difference between shallow and deep copy)
    // this is true for all types that implement the Copy trait (they don't move)
    
    // capacity is how much space a variable on the heap has been allocated
    // length is how much of that memory the variable is currently using
    
    println!("{}", s);
}
