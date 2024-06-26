// Const::Ty is not supported (constant.rs:6:38)
// fixed
fn main() {
    let n = 0;
    match n {
        0..=9 => false,
        _ => true,
    };
}