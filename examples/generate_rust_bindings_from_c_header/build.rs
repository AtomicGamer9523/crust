const BINDINGS: &str = "bindings.h";
const LIB: &str = "clib";

fn main() {
    println!("cargo:rerun-if-changed={BINDINGS}");
    println!("cargo:rustc-link-lib=dylib={LIB}");
}