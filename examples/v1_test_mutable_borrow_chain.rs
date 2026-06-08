// v1.0.0 测试：可变借用链
fn main() {
    let mut x = 5;
    let r1 = &mut x;
    *r1 += 1;
    
    let r2 = &mut x;  // 新的可变借用，r1 已过期
    *r2 += 2;
    
    println!("x = {}", x);
}
