const DOC_TESTS_GLOB: &str = "resources/test_docs/*.md";

fn main() {
    println!("cargo:rerun-if-changed={DOC_TESTS_GLOB}");
}
