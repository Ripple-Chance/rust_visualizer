// v1.0.0 测试：函数参数所有权转移
fn take_ownership(s: String) {
    println!("{}", s);
} // s 在此处被 drop

fn borrow_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s1 = String::from("hello");
    take_ownership(s1);  // 所有权转移给函数
    
    let s2 = String::from("world");
    borrow_string(&s2);  // 借用
    println!("{}", s2);  // s2 仍有效
}
