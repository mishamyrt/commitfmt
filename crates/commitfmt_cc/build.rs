const DOC_TESTS_GLOB: &str = "resources/doc_tests/*.md";

fn main() {
    println!("cargo:rerun-if-changed={DOC_TESTS_GLOB}");
}
