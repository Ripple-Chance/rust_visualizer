fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len();
    let average = sum / count as i32;
    
    println!("Sum: {}", sum);
    println!("Count: {}", count);
    println!("Average: {}", average);
}
