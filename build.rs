//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.
use std::fs;

fn main() {
    // // Put `memory.x` in our output directory and ensure it's
    // // on the linker search path.
    // let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // File::create(out.join("memory.x"))
    //     .unwrap()
    //     .write_all(include_bytes!("memory.x"))
    //     .unwrap();
    // println!("cargo:rustc-link-search={}", out.display());

    // // By default, Cargo will re-run a build script whenever
    // // any file in the project changes. By specifying `memory.x`
    // // here, we ensure the build script is only re-run when
    // // `memory.x` is changed.
    // println!("cargo:rerun-if-changed=memory.x");
    let files: Vec<std::path::PathBuf> = fs::read_dir("vendored/xm125/Src")
        .unwrap()
        .filter(|f| f.as_ref().unwrap().path().ends_with(".c"))
        .map(|f| f.unwrap().path())
        .collect();

    cc::Build::new()
        .file("vendored/xm125/Src/examples/getting_started/example_bring_up.c")
        .file("vendored/xm125/Src/integration/acc_hal_integration_stm32cube_xm.c")
        .include("vendored/xm125/Inc")
        .compile("xm125");

    println!("cargo:rustc-link-lib=acconeer_a121");
    println!("cargo:rustc-link-lib=acc_detector_distance_a121");
    println!(
        "cargo:rustc-link-search=/home/milesg/Projects/garage-backup-sensor/vendored/xm125/lib"
    );
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
