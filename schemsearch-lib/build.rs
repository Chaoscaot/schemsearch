use cc;

fn main() {
    cc::Build::new()
        .file("src/compare.c")
        .flag("-ftree-vectorize")
        .flag("-march=native")
        .flag("-mtune=native")
        .flag("-ffast-math")
        .compile("compare");
}