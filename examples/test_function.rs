fn process_data(a: i32, mut b: String) -> String {
    let result = format!("{}-{}", a, b);
    result
}

fn main() {
    let data = String::from("test");
    let output = process_data(42, data);
    println!("{}", output);
}
