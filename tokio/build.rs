extern crate cmake;
use cmake::Config;

fn main() {
    // TODO: support all 3 platforms for building libverona
    let dst = Config::new("libverona").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=verona");
    if cfg!(any(target_os = "macos", target_os = "freebsd")) {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
        println!("cargo:rustc-link-lib=static=c++");
    }
}