// v1.0.0 测试：借用冲突检测
fn main() {
    let mut data = String::from("hello");
    
    let r1 = &data;       // 不可变借用
    let r2 = &data;       // 另一个不可变借用（允许）
    println!("{} {}", r1, r2);
    
    let r3 = &mut data;   // 可变借用
    r3.push_str(" world");
    println!("{}", r3);
}
