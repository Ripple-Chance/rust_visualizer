fn main() {
    let name = String::from("Alice");
    let age = 25;
    let is_student = true;
    
    let mut score = 100;
    score += 10;
    
    let info = format!("{} is {} years old", name, age);
    println!("{}", info);
    println!("Score: {}", score);
    println!("Student: {}", is_student);
}
