# Double Mutable Borrow in Rust
This example demonstrates a common error in Rust: creating multiple mutable references to the same variable. This leads to a runtime panic because Rust enforces strict borrowing rules to prevent data races.

The `bug.rs` file contains the buggy code, while `bugSolution.rs` shows how to fix it.  The key is to understand and avoid situations where multiple mutable references point to the same data simultaneously.