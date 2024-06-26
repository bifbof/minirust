fn _foo() {}

fn main() {
    // fails due to Cast(PointerCoercion(UnsafeFnPointer)) not being supported
    // let _ = foo as unsafe fn();

    let _ = u32::BITS;

    // currently fails in well-formedness check
    // let _ = foo;

    // fails as _p is `Ptr(FnPtr(Rust))` while RHS is `Ptr(FnPtr(C))`
    // let _p = foo as fn(); // as usize;

    // ty translation fails TyKind Closure not supported (ty.rs:88:18)
    // let _unit = || ();
    
    // two non-deterministic failures
    // 1. rvalue translation fails for NullaryOp(SizeOf, Ty {..}) (rvalue.rs:223:18)
    // 2. terminator translation fails due to UnwindResume (bb.rs:163:18)
    // let _val = Box::new(());

    // fails as unsized type (ty.rs:6:9)
    // let _a = "unsized type";

    // fails in execution as `predict` is not implemented in libspecr
    // let vec = 1u8;
    // let x = 42 as *mut u8;
    // unsafe { *x = vec; }

    // bool relative comparison fail as not yet supported
    // let _ = true == true;
    // binop (dereferencing pointer) fail due to CheckedBinaryOp
    // let _ = &0 + 0;

    // some smir problems not being to implement a defid
    // let b = Box::new(0);
    // let x = (&*b as *const i32).wrapping_sub(0x800); // out-of-bounds

}
