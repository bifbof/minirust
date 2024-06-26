//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows
//@compile-flags: -Zmiri-permissive-provenance
#![feature(strict_provenance, exposed_provenance)]

use std::ptr;

/// Ensure that we can move between allocations after casting back to a ptr
fn main() {
    let x: i32 = 0;
    let y: i32 = 1;
    // so here we assign x and y: 0 and 1

    let x_ptr = &x as *const i32;
    let y_ptr = &y as *const i32;
    // we create two pointers based on x and y

    let x_usize = x_ptr.addr();
    let ptr = ptr::with_addr::<i32>(x_usize);
    let y_usize = y_ptr.addr();


    let ptr = ptr::with_exposed_provenance::<i32>(y_usize);
    let ptr = ptr.with_addr(x_usize);
    assert!(unsafe { *ptr } == 0);
}
