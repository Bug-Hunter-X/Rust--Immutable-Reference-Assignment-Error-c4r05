# Rust Immutable Reference Assignment Error

This example demonstrates a common error in Rust: trying to assign a value to an immutable reference.  Rust's borrow checker prevents data races and ensures memory safety.  Attempting to modify a value through an immutable reference violates this safety. The solution is to use mutable references.