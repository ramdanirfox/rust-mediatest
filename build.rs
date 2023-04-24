fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = manifest_dir + "\\lib";
    println!("Hallo dunia!");
    // Add the directory containing the libopenmpt header files to the include path
    println!("cargo:include=lib\\inc");

    // Add the directory containing the libopenmpt_ext header files to the include path
    println!("cargo:include=lib\\inc");

    // Link with the libopenmpt static library
 
    // println!("cargo:rustc-link-lib=libopenmpt");
    println!("cargo:rustc-link-search=native={}", lib_dir);
    // println!("cargo:rustc-link-arg={}", lib_dir + "\\dll\\libopenmpt.dll");
    println!("cargo:rustc-link-lib=openmpt");

    // Tell Rust to rebuild the project if the library changes
    // println!("cargo:rerun-if-changed=lib\\libopenmpt.lib");
}