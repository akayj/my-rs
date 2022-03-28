fn main() {
    let lib_path = "/Users/yj/macos/lib";
    let lib_name = "cpuid";

    // use *cc* crate
    cc::Build::new().file("foo.c").compile("foo");

    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=dylib={}", lib_name);

    println!("cargo:rerun-if-changed=foo.c");
}
