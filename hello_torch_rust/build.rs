// TODO: change `.cargo/config.toml` too 
const RELATIV_PATH: &str = "..\\libtorch\\libtorch\"";

fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // $Env:LIBTORCH = "C:\\Users\\Aurel\\OneDrive\\Documents\\dev2\\torch\\libtorch\\libtorch"
    println!("cargo:rustc-env=LIBTORCH=\"{dir}\\{RELATIV_PATH}");

    let path = std::env::var("Path").unwrap();
    // $Env:Path += ";C:\\Users\\Aurel\\OneDrive\\Documents\\dev2\\torch\\libtorch\\libtorch\\lib"
    println!("cargo:rustc-env=PATH=\"{path};{dir}\\{RELATIV_PATH}\\lib;\"");

    println!("cargo:rerun-if-changed=build.rs");
}


