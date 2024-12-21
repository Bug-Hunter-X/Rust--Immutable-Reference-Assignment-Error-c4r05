fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    
    *y = 10; // Modifying x through y is allowed
    println!("x = {}", x); // Prints x = 10
} 