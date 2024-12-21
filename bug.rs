fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y = 10; // Modifying x through y is allowed
    // *z = 20; // This is an error - Cannot assign to immutable reference
    println!("x = {}", x); // Prints x = 10
}