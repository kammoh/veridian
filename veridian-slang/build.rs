use flate2::read::GzDecoder;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;

fn download_slang() -> Result<(), Box<dyn std::error::Error>> {
    let target = "https://github.com/MikePopoloski/slang/archive/refs/tags/v4.0.tar.gz";
    let fname = "slang-4.0.tar.gz";
    let mut response = reqwest::blocking::get(target)?;
    let mut dest = File::create(fname)?;
    response.copy_to(&mut dest)?;
    let tar = GzDecoder::new(File::open(fname)?);
    let mut archive = Archive::new(tar);
    archive.unpack("slang_wrapper/.")?;
    Ok(())
}

fn build_slang_wrapper(slang_dir: &PathBuf) {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++20")
        .cpp_set_stdlib(Some("c++"))
        .flag("-Wno-type-limits")
        // .static_flag(true)
        .include("/usr/local/include")
        // .include(format!("-I{}", slang_dir.join("include").display()))
        // .include("/opt/homebrew/include")
        .include("/opt/homebrew/opt/boost/include")
        .file("slang_wrapper/src/slang_lib.cpp")
        .file("slang_wrapper/src/basic_client.cpp")
        .compile("slangwrapper");
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=slang_wrapper/src/slang_wrapper.h");
    println!("cargo:rerun-if-changed=slang_wrapper/src/slang_lib.cpp");

    // download_slang().unwrap();

    let slang_dir = PathBuf::from("/Volumes/src/slang/build");
    build_slang_wrapper(&slang_dir);

    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        // .clang_arg(format!("-I{}", slang_dir.join("include").display()))
        // .clang_arg(format!(
        //     "-I{}",
        //     slang_dir.join("_deps/fmt-src/include").display()
        // ))
        .clang_arg("-I/usr/local/include")
        // .clang_arg("-I/opt/homebrew/opt/boost/include")
        // .clang_arg("-I/opt/homebrew/include")
        .header("slang_wrapper/src/slang_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // println!(
    //     "cargo:rustc-link-search=native={}/slang_wrapper/slang/lib",
    //     env::var("CARGO_MANIFEST_DIR").unwrap()
    // );
    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     slang_dir.join("lib").display()
    // );
    // println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=slangwrapper");
    println!("cargo:rustc-link-lib=static=svlang");
    println!("cargo:rustc-link-lib=static=mimalloc");
    println!("cargo:rustc-link-lib=static=fmt");
    // println!("cargo:rustc-link-lib=dylib=stdc++");

    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
