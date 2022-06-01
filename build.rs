fn main() {
    let src = [
        "src/clib.c",
    ];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("include")
        .flag("-Wno-unused-parameter");

    build.compile("clib");
}
