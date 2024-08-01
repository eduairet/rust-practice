fn main() {
    cc::Build::new().file("src/hello.c").compile("hello");

    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");
}
