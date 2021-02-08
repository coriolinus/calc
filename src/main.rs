#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("This binary was built without the `cli` feature. It does nothing.");
    std::process::exit(1);
}

#[cfg(feature = "cli")]
fn main() {
    unimplemented!()
}
