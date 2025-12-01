use std::process;
use std::env;
use std::thread::available_parallelism;

fn main() {
    let cpp_archive: &str = "erals";
    let cpp_out_dir: &str = "cpp_deps";

    let cpu_threads: usize = match available_parallelism() {
        Ok(unwrapped) => unwrapped.get(),
        Err(_) => 1,
    };
    let out_dir: String = env::var("OUT_DIR").expect("OUT_DIR not set");

    let status = process::Command::new("make")
        .arg(format!("-j{}", cpu_threads))
        .arg(format!("archive_name={}", cpp_archive))
        .arg(format!("obj_dir_prefix={}", out_dir))
        .arg(format!("obj_dir_suffix={}", cpp_out_dir))
        .status()
        .expect("make command failed to run; cannot build");

    if !status.success() {
        process::exit(status.code().unwrap());
    }

    println!("cargo:rustc-link-search=native={}/{}", out_dir, cpp_out_dir);
    println!("cargo:rustc-link-lib=static={}", cpp_archive);
    println!("cargo:rustc-link-lib=dylib=stdc++");
}

