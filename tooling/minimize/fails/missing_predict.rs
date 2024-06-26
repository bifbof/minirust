fn main() {
    let vec = 1u8;
    let x = 42 as *mut u8;
    unsafe {
        *x = vec;
    }
}
