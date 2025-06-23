use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

pub struct TestCase {
    pub name: String,
    pub message: String,
}

fn get_case_name(group: &str, case_path: &PathBuf) -> String {
    let mut path = PathBuf::from(group);
    path.push(case_path);
    let filename = path.file_name().unwrap().to_string_lossy().to_string();
    filename.split('.').next().unwrap().to_string()
}

pub fn read_cc_files() -> io::Result<Vec<TestCase>> {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("resources/cc");

    let mut entries: Vec<_> = fs::read_dir(&dir)?
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .collect();

    entries.sort_by_key(std::fs::DirEntry::file_name);

    let mut files = Vec::with_capacity(entries.len());
    for entry in entries {
        let mut content = String::new();
        fs::File::open(entry.path())?.read_to_string(&mut content)?;
        files.push(TestCase { name: get_case_name("cc", &entry.path()), message: content });
    }
    Ok(files)
}
