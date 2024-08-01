fn main() {
    cc::Build::new().file("src/hello.c").compile("hello");

    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");

    cc::Build::new()
        .define("APP_NAME", "\"bar\"")
        .define(
            "VERSION",
            format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
        )
        .define("WELCOME", None)
        .file("src/bar.c")
        .compile("bar");
}
