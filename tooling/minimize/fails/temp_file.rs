#![feature(strict_provenance)]

fn main() {
    let x_ptr = &0 as *const i32;
    let x_usize = x_ptr.addr();
    let _ = x_ptr.with_addr(x_usize);
}
