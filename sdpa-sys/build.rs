extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
fn lapack_cmake() -> Result<std::path::PathBuf, ()> {
    use cmake::Config;
    /*
        // -L flag相当
        println!("cargo:rustc-link-search=C:\\Program Files (x86)\\Microsoft SDKs\\MPI\\Lib\\x64");

        // -l flag相当
        println!("cargo:rustc-link-lib=msmpi");

        let dst = Config::new("src/lapack-3.10.1")
                         //.define("FOO", "BAR")
                         //.cflag("-foo")
                         .generator("MinGW Makefiles")
                         //.define("NOFORTRAN", "0")
                         .build();
        println!("cargo:rustc-link-search=native={}", dst.display());
    */

    let dst = Config::new("src/mumps-5.5.1.7/scripts")
        //.define("FOO", "BAR")
        //.cflag("-foo")
        .no_build_target(true)
        .generator("MinGW Makefiles")
        //					 .configure_arg("-S src/mumps-5.5.1.7/scripts")
        //					 .define("parallel", "false")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    Ok(dst)
}

/// required from `sdpa`
///
/// # pre required
///
///  * LAPACK
///
///
#[cfg(target_os = "windows")]
fn dmumps_cmake(vcpkg_path: Option<std::path::PathBuf>) -> Result<std::path::PathBuf, ()> {
    use cmake::Config;

    let dst = Config::new("src/mumps-5.5.1.7")
        //.define("FOO", "BAR")
        //.cflag("-foo")
        .generator("MinGW Makefiles")
/*        .define(
            "CMAKE_TOOLCHAIN_FILE",
            format!(
                "{}/scripts/buildsystems/vcpkg.cmake",
                vcpkg_path.unwrap().display()
            ),
        )*/
        .define("CMAKE_C_COMPILER", "gcc")
        .define("CMAKE_CXX_COMPILER", "g++")
        .define("MKLROOT", env::var("ONEAPI_ROOT").unwrap() + "/mkl")
        .define(
            "MPIEXEC_EXECUTABLE",
            env::var("ONEAPI_ROOT").unwrap() + "mpi/latest/bin",
        )
        .define("MPI_HOME", env::var("ONEAPI_ROOT").unwrap() + "/mpi")
        .define("I_MPI_ROOT", env::var("ONEAPI_ROOT").unwrap() + "/mpi")
        //.define("BLA_VENDOR", "Intel10_64lp")
        /*        .define("BLA_VENDOR", "OpenBLAS")
        .define(
            "CMAKE_PREFIX_PATH",
            format!("{}", blas_path.clone().unwrap().display()),
        )*/
        //.define("NOFORTRAN", "0")
        .define("CMAKE_FIND_DEBUG_MODE", "1")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    Ok(dst)
}

#[cfg(target_os = "windows")]
fn open_blas_cmake() -> Result<std::path::PathBuf, ()> {
    use cmake::Config;

    let dst = Config::new("src/OpenBLAS-0.3.21")
        //.define("FOO", "BAR")
        //.cflag("-foo")
        .generator("MinGW Makefiles")
        .define("NOFORTRAN", "0")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    Ok(dst)
}

#[cfg(target_os = "windows")]
fn vcpkg_install() -> Result<std::path::PathBuf, ()> {
    use std::process::Command;

    let vcpkg_src_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("vcpkg");
    // Cloning from git
    Command::new("git")
        .args([
            "clone",
            "https://github.com/Microsoft/vcpkg.git",
            vcpkg_src_path.display().to_string().as_str(),
        ])
        .output()
        .expect("failed to execute process. (git clone)");

    // Setting up `vcpkg`
    Command::new(format!("{}/bootstrap-vcpkg.bat", vcpkg_src_path.display()).as_str())
        .output()
        .expect("failed to execute process. (installijng vcpkg)");

    Command::new(format!("{}/vcpkg.exe", vcpkg_src_path.display()).as_str())
        .args(["install", "pkgconf"])
        .output()
        .expect("failed to execute process. (installing pkgconfig)");
    Ok(vcpkg_src_path)
}

fn main() {
    // -L flag相当
    //	println!("cargo:rustc-link-search=./src/sdpa-7.3.9");

    // -l flag相当
    println!("cargo:rustc-link-lib=stdc++");
    //	println!("cargo:rustc-link-lib=[kind="static"]NAME Temp");

    // other flag
    //	println!("cargo:rustc-flags=FLAGS Temp");

    // other flag
    //	println!("cargo:rustc-cdylib-link-arg=FLAG Temp");

    //	let lapack_dst = lapack_cmake().unwrap();
    //let blas_dst = open_blas_cmake().unwrap();
//    let vcpkg_dst = vcpkg_install().unwrap();
    let dmumps_dst = dmumps_cmake(None/*Some(vcpkg_dst)*/).unwrap();

    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .cpp_link_stdlib("stdc++")
        .flag("-Wall")
        .flag("-std=gnu++11")
        .flag("-fmessage-length=0")
        .include("src/sdpa-7.3.9")
        .include(format!("{}/include", dmumps_dst.display()))
        //		.include(format!("{}/include", lapack_dst.display()))
        //.include(format!("{}/include", blas_dst.display()))
        .file("src/sdpa-7.3.9/sdpa_block.cpp")
        .file("src/sdpa-7.3.9/sdpa_call.cpp")
        .file("src/sdpa-7.3.9/sdpa_chordal.cpp")
        .file("src/sdpa-7.3.9/sdpa_dataset.cpp")
        .file("src/sdpa-7.3.9/sdpa_dpotrf.cpp")
        //		.file("src/sdpa-7.3.9/sdpa_exe.cpp")
        .file("src/sdpa-7.3.9/sdpa_io.cpp")
        .file("src/sdpa-7.3.9/sdpa_jordan.cpp")
        .file("src/sdpa-7.3.9/sdpa_linear.cpp")
        .file("src/sdpa-7.3.9/sdpa_newton.cpp")
        .file("src/sdpa-7.3.9/sdpa_parts.cpp")
        .file("src/sdpa-7.3.9/sdpa_solve.cpp")
        .file("src/sdpa-7.3.9/sdpa_struct.cpp")
        .file("src/sdpa-7.3.9/sdpa_tool.cpp")
        .compile("sdpa_sys");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/sdpa-7.3.9/sdpa_call.h")
        .allowlist_type("SDPA")
        .clang_arg("-I src/sdpa-7.3.9")
        .clang_arg("-x")
        .clang_arg("c++")
        //.clang_arg("-std=c++11")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
