// Fails due to Constant fn not being translated.
fn main() {
    let f: fn(()) -> Option<()> = Some::<()>;
    f(());
}
