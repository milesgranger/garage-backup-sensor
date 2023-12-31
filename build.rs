fn main() {
    println!("cargo:rustc-link-lib=acconeer_a121");
    println!("cargo:rustc-link-lib=acc_detector_distance_a121");
    println!("cargo:rustc-link-lib=acc_detector_presence_a121");
    println!("cargo:rustc-link-search=vendored/xm125/lib");
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
