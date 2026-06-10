// Test case: Nested scopes
fn main() {
    let a = 1;
    {
        let b = 2;
        let c = a + b;
        {
            let d = 3;
            let _e = c + d;
        }
    }
    let _f = a * 2;
}
