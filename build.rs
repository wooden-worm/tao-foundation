fn main() {
    let target = std::env::var("TARGET").unwrap();

    if target.contains("-apple-") {
        println!("cargo:rustc-link-lib=framework=Foundation");
    }
}
