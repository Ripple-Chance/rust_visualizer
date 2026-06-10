// Test case: Ownership transfer through functions
fn take_ownership(s: String) -> String {
    println!("Taking ownership of: {}", s);
    s
}

fn borrow_reference(s: &String) {
    println!("Borrowing: {}", s);
}

fn main() {
    let s1 = String::from("hello");
    let s2 = take_ownership(s1);
    
    borrow_reference(&s2);
    
    let s3 = s2;
    println!("s3: {}", s3);
}