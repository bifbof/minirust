// transmute call fails in rvalue:223:18
fn main() {
    unsafe {
        let p: *const u8 = ::std::mem::transmute(0_usize);
    }
}
