fn f() {}
fn main() {
    let _ = f as unsafe fn();
}
