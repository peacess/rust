fn main() {
    cc::Build::new()
        .cpp(true)
        // .cpp_link_stdlib("stdc++")
        .warnings(false)
        .static_flag(true)
        .static_crt(true)
        .flag("-static-libstdc++")
        // .flag("-C target-feature=+crt-static")
        .flag("--std=c++14")
        .file("cpp/sample.cpp")
        .compile("cpp");

    let bindings = bindgen::Builder::default()
        .header("cpp/data.h")
        .header("cpp/sample.h")
        .clang_args(["-x", "c++", "--std=c++14"]) //, "-fkeep-inline-functions"
        .enable_cxx_namespaces()
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file("src/gen/cpp_generate.rs").unwrap();
}
