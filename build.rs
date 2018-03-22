use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const LIB_SDL: &str = "/usr/local/lib/libSDL2.so";

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.contains("unknown-linux") {
        // HACK: Set up a host-accesible writable path to copy libSDL2.so when using cross
        let target_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap());
        let mut lib_dir = target_dir.clone();
        lib_dir.push(target.clone());
        lib_dir.push("release");
        lib_dir.push("lib");

        let lib_sdl_path = fs::canonicalize(LIB_SDL).unwrap();

        let mut new_file_path = lib_dir.clone();
        new_file_path.push("libSDL2.so");

        let _ = fs::create_dir_all(&lib_dir);

        fs::copy(lib_sdl_path.as_path(), new_file_path.as_path())
            .expect("Couldn't copy libSDL2.so");
    }

    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        } else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path())
                        .expect("Can't copy from DLL dir");
                }
            }
        }
    }
}
