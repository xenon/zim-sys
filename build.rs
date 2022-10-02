use cxx_build::CFG;
use std::path::PathBuf;

#[cfg(target_family = "unix")]
fn find_system_lib() {
    let libzim = pkg_config::probe_library("libzim").unwrap();

    println!("cargo:rustc-link-search=native={:?}", libzim.link_paths[0]);

    let include_paths = libzim.include_paths.iter().map(PathBuf::as_path);
    CFG.exported_header_dirs.extend(include_paths);
}

#[cfg(not(target_family = "unix"))]
fn find_system_lib() -> Vec<String> {
    println!("cargo:rustc-link-lib=zim");
    vec![]
}

fn main() {
    find_system_lib();

    let sources = ["src/binding.rs"];
    cxx_build::bridges(sources)
        .file("zim-bind.cc")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-deprecated-declarations")
        .compile("zim-sys");

    println!("cargo:rustc-link-lib=zim"); // if this doesn't go after cxx_build.(...).compile() we get a link error
}
