// fails due to MutToConst cast not being supported
fn main() {
    let b = &42 as *const i32;
    let _ = b as *mut i32;
}