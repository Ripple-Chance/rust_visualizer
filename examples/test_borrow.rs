// Test case: Borrow relationships
fn main() {
    let s1 = String::from("hello");
    let r1 = &s1;          // Immutable borrow
    let r2 = &s1;          // Another immutable borrow
    println!("{} {}", r1, r2);

    let mut s2 = String::from("world");
    let mr1 = &mut s2;    // Mutable borrow
    println!("{}", mr1);
    mr1.push_str("!");
}
