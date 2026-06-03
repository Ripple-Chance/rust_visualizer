fn main() {
    let x = 42;
    let mut y = String::from("hello");
    
    {
        let z = x + 1;
        y.push_str(" world");
        println!("{}", z);
    }
    
    println!("{}", x);
    println!("{}", y);
}

fn process(a: i32, mut b: String) -> String {
    let result = a.to_string();
    b.push_str(&result);
    b
}
