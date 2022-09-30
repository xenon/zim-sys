use std::{env, path::PathBuf};

#[cfg(target_family = "unix")]
fn find_system_lib() -> Vec<String> {
    let package = pkg_config::Config::new().probe("libzim").unwrap();
    println!("cargo:rustc-link-search=native={:?}", package.link_paths[0]);
    println!("cargo:rustc-link-lib=zim");

    let mut includes = package.include_paths.clone();
    includes
        .iter_mut()
        .map(|x| {
            if !x.ends_with("include") {
                x.pop();
            }
            x
        })
        .map(|x| x.to_string_lossy())
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

#[cfg(not(target_family = "unix"))]
fn find_system_lib() -> Vec<String> {
    println!("cargo:rustc-link-lib=zim");
    vec![]
}

fn bindings(includes: &[String]) -> bindgen::Bindings {
    let mut bindings = bindgen::Builder::default()
        .enable_cxx_namespaces()
        .header("wrapper.h")
        .clang_args(&["-x", "c++"])
        // archive.h
        .allowlist_type("zim::Archive")
        // blob.h
        .allowlist_type("zim::Blob")
        // entry.h
        .allowlist_type("zim::Entry")
        // item.h
        .allowlist_type("zim::Item")
        // search.h
        .allowlist_type("zim::Searcher")
        .allowlist_type("zim::Query")
        .allowlist_type("zim::Search")
        .allowlist_type("zim::SearchResultSet")
        // search_iterator.h
        .allowlist_type("zim::SearchIterator")
        // suggestion.h
        .allowlist_type("zim::SuggestionSearcher")
        .allowlist_type("zim::SuggestionSearch")
        .allowlist_type("zim::SearchResultSet")
        // suggestion_iterator.h
        .allowlist_type("zim::SearchIterator")
        .allowlist_type("zim::Uuid")
        // version.h
        .allowlist_type("zim::LibVersions")
        .allowlist_function("zim::getVersions")
        // zim.h
        .allowlist_type("zim::Compression");

    for inc in includes {
        bindings = bindings.clang_arg(format!("-I{}", *inc));
    }

    bindings.generate().expect("failed to generate bindings!")
}

fn main() {
    let includes = find_system_lib();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings(&includes)
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings to file!");
}
