// two non-deterministic failures
// 1. rvalue translation fails for NullaryOp(SizeOf, Ty {..}) (rvalue.rs:223:18)
// 2. terminator translation fails due to UnwindResume (bb.rs:163:18)
fn main() {
    let _val = Box::new(());
}
