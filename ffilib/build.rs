use cxx_build::CFG;
fn main() {
    CFG.include_prefix = "";
    let opencv = pkg_config::probe_library("opencv4").unwrap();
    cxx_build::bridge("src/lib.rs")
        .file("fficpp/src/fficpp/cvfn.cpp")
        .include("fficpp/include")
        .includes(opencv.include_paths)
        .compile("ffilib");

    for link_path in opencv.link_paths {
        println!("cargo:rustc-link-search={}", link_path.to_str().unwrap());
    }
    for lib in opencv.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}
