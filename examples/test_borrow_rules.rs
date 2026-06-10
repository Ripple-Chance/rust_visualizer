// Test case: Borrow rules - multiple borrows
fn main() {
    let mut x = 10;
    let y = &x;
    let z = &x;
    
    println!("y: {}, z: {}", y, z);
    
    let w = &mut x;
    *w += 5;
    println!("x: {}", x);
}