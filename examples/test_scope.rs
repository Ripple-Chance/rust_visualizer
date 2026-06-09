// Test case: Nested scopes
fn main() {
    let a = 1;
    {
        let b = 2;
        let c = a + b;
        {
            let d = 3;
            let e = c + d;
        }
    }
    let f = a * 2;
}
