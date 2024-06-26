//S some smir problems not being to implement a defid
fn main() {
    let b = Box::new(0);
    let x = (&*b as *const i32).wrapping_sub(0x800); // out-of-bounds
}
