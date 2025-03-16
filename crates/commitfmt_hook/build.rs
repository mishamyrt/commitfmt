use minijinja::{context, Environment};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const HOOK_TPL_PATH: &str = "resources/prepare-commit-msg.j2.sh";
const HOOK_OUT_NAME: &str = "hook_content.rs";

fn get_os() -> String {
    match env::consts::OS {
        "macos" => "darwin".to_string(),
        "windows" | "linux" | "freebsd" | "openbsd" => env::consts::OS.to_string(),
        _ => panic!("Unsupported OS"),
    }
}

fn get_arch() -> String {
    match env::consts::ARCH {
        "x86_64" => "x86_64".to_string(),
        "aarch64" => "arm64".to_string(),
        _ => panic!("Unsupported architecture"),
    }
}

fn get_out_path() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    Path::new(&out_dir).join(HOOK_OUT_NAME)
}

fn main() {
    let Ok(template_str) = fs::read_to_string(HOOK_TPL_PATH) else {
        panic!("Unable to read template file: {HOOK_TPL_PATH}");
    };
    // Setup renderer
    let mut env = Environment::new();
    env.add_template(HOOK_TPL_PATH, &template_str).unwrap();

    // Render template
    let template = env.get_template(HOOK_TPL_PATH).unwrap();
    let hook_content = template
        .render(context! {
            extension => env::consts::EXE_EXTENSION.to_string(),
            os => get_os(),
            arch => get_arch(),
        })
        .unwrap();

    // Write to file
    let dest_path = get_out_path();
    let Ok(mut file) = File::create(&dest_path) else {
        panic!("Unable to create file: {dest_path:?}");
    };
    writeln!(file, "pub const HOOK_CONTENT: &str = {hook_content:?};")
        .expect("Unable to write to file");

    // Add rerun-if-changed
    println!("cargo:rerun-if-changed={}", dest_path.display());
    println!("cargo:rerun-if-changed=build.rs");
}
