// Test case: Nested scopes with ownership transfer
fn main() {
    let outer = String::from("outer");
    
    {
        let inner = String::from("inner");
        let borrowed = &inner;
        println!("{}", borrowed);
    }
    
    let another = outer;
    println!("{}", another);
}