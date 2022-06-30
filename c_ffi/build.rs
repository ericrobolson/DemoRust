fn main() {
    cc::Build::new()
        .include("c_src/bar.h")
        .file("c_src/bar.c")
        .compile("foo");
}
