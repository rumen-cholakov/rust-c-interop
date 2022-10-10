// update build.rs file as:
extern crate cc;

fn main() {
    cc::Build::new()
        // .file("src/doubler.c")
        .cpp(true)
        .file("src/doubler.cpp")
        .compile("libdoubler.a");
}