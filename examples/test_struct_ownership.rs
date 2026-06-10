// Test case: Struct ownership
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p1 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let _p2 = p1;
    
    let mut p3 = Person {
        name: String::from("Bob"),
        age: 25,
    };
    
    p3.age = 26;
    let borrowed_name = &p3.name;
    
    println!("Name: {}", borrowed_name);
}