// currently fails in well-formedness check
fn main() {
    let _fn_ptr: fn() = foo;
}

fn foo() {}
