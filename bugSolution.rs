fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1;  // Modifying x through y
    }
    {
        let z = &mut x; // z is a mutable reference to x
        *z += 1; //Modifying x through z
    }

    println!("x = {}", x);
} 