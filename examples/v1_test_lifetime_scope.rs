// v1.0.0 测试：生命周期与作用域
fn main() {
    let outer = 10;
    {
        let inner = 20;
        let ref_inner = &inner;  // 引用 inner
        println!("inner: {}, ref: {}", inner, ref_inner);
    } // inner 和 ref_inner 在此处结束生命周期
    
    println!("outer: {}", outer);
}
