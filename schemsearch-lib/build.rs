use cc;

fn main() {
    cc::Build::new()
        .file("src/compare.c")
        .compile("compare");
}