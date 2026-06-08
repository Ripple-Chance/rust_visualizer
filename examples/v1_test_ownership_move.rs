// v1.0.0 测试：所有权转移
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // 所有权从 s1 转移到 s2
    
    println!("{}", s2);
    // s1 在此处已无效
}
