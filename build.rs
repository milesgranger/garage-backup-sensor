use bindgen::builder;
use std::{fs, path::PathBuf};

fn main() {
    // math and c libs, TODO: better link search path finding, besides hardcoded?
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-search=/usr/arm-none-eabi/lib");

    // a121 libs
    println!("cargo:rustc-link-lib=static=acconeer_a121");
    println!("cargo:rustc-link-lib=static=acc_detector_distance_a121");
    println!("cargo:rustc-link-lib=static=acc_detector_presence_a121");
    println!("cargo:rustc-link-search=vendored/cortex_m4_gcc/rss/lib");

    // embedded stuffy stuff
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}

// Commented out, using Makefile bindgen cli as this tends to fail on inttypes.h missing
// but that is available in /usr/arm-none-eabi/include but haven't tried adding that
// as it's working now from the CLI approach; if that fails maybe it's worth a try here.
#[allow(dead_code)]
fn run_bindgen() {
    // Also need to add this as a mod in src/a121/mod.rs
    let headers = fs::read_dir("vendored/cortex_m4_gcc/rss/include")
        .unwrap()
        .filter_map(|v| {
            v.ok()
                .map(|p| {
                    let path = p.path();
                    if path.to_str().unwrap().ends_with(".h") {
                        Some(path)
                    } else {
                        None
                    }
                })
                .flatten()
        })
        .collect::<Vec<PathBuf>>();
    if headers.is_empty() {
        panic!("Didn't find any headers!");
    }
    for header in headers {
        println!("Generating bindings to header: {}", header.display());
        let bindings = builder()
            .header(format!("{}", header.display()))
            .use_core()
            // .ctypes_prefix("cty")
            .generate()
            .unwrap();

        bindings
            .write_to_file(format!("src/a121/{}.rs", &header.to_str().unwrap()))
            .unwrap();
    }
}
