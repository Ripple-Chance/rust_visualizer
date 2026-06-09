// Test case: Ownership move semantics
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;          // s1 moved to s2
    // println!("{}", s1);  // Error: s1 is moved
    println!("{}", s2);

    let v1 = vec![1, 2, 3];
    let v2 = v1;          // v1 moved to v2
    // println!("{:?}", v1); // Error: v1 is moved
    println!("{:?}", v2);

    let x = 5;
    let y = x;            // Copy: x is copied to y
    println!("{} {}", x, y);
}
