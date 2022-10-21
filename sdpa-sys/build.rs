
extern crate cc;

fn main()
{

// -L flag相当
//	println!("cargo:rustc-link-search=./src/sdpa-7.3.9");

// -l flag相当
	println!("cargo:rustc-link-lib=stdc++");
//	println!("cargo:rustc-link-lib=[kind="static"]NAME Temp");

// other flag
//	println!("cargo:rustc-flags=FLAGS Temp");

// other flag
//	println!("cargo:rustc-cdylib-link-arg=FLAG Temp");



	cc::Build::new()
		.cpp(true)
		.warnings(true)
		.cpp_link_stdlib("stdc++")
		.flag("-Wall")
		.flag("-std=gnu++11")
		.flag("-fmessage-length=0")
		.include("src/sdpa-7.3.9")
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

}