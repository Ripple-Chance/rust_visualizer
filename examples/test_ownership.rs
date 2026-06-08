fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // ownership move
    
    let s3 = String::from("world");
    let r1 = &s3;  // immutable borrow
    let r2 = &s3;  // another immutable borrow
    
    println!("{} {} {}", s2, r1, r2);
    
    let mut x = 10;
    let rx = &mut x;  // mutable borrow
    *rx += 5;
    
    println!("{}", x);
}
