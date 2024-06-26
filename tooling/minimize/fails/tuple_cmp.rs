// binary ops for other types fail in **well-formed check**
// strangely here it happens only if tuple size >= 3
fn main() {
    let _ = (1, 2, 3) == (1, 2, 3);
}