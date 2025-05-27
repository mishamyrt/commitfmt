use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Write;
use std::fs::{self, create_dir_all};
use std::path::Path;
use syn::{File, Item};
use walkdir::WalkDir;

#[derive(Clone)]
struct RuleInfo {
    rule_name: String,
    file_name: String,
    message: String,
    fix_mode: String,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/rules");

    println!("cargo:warning=Building rules documentation...");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rules_dir = Path::new(&manifest_dir).join("src").join("rules");
    let docs_dir = Path::new(&manifest_dir).join("docs");

    // Create a map to store rules by group
    let mut rules_by_group: HashMap<String, Vec<RuleInfo>> = HashMap::new();
    let mut processed_files = HashSet::new();

    for entry in WalkDir::new(&rules_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "rs" && !path.file_name().unwrap().to_str().unwrap().starts_with("mod") {
                process_file(
                    path,
                    &rules_dir,
                    &docs_dir,
                    &mut rules_by_group,
                    &mut processed_files,
                );
            }
        }
    }

    // Generate index file
    generate_index_file(&docs_dir, &rules_by_group);
}

fn process_file(
    file_path: &Path,
    rules_dir: &Path,
    docs_dir: &Path,
    rules_by_group: &mut HashMap<String, Vec<RuleInfo>>,
    processed_files: &mut HashSet<String>,
) {
    let content = fs::read_to_string(file_path).unwrap();
    let syntax: File = syn::parse_file(&content).unwrap();

    let relative_path = file_path.strip_prefix(rules_dir).unwrap();
    let components: Vec<_> = relative_path.components().collect();

    if components.is_empty() {
        return;
    }

    let group = components[0].as_os_str().to_str().unwrap().to_string();
    // Calculate the source path relative to src/rules
    let source_path =
        format!("src/rules/{}", relative_path.display().to_string().replace('\\', "/"));

    for item in syntax.items {
        if let Item::Struct(s) = item {
            let struct_name = s.ident.to_string();

            // Skip if we've already processed this file+struct
            let file_struct_key = format!("{}:{}", file_path.display(), struct_name);
            if processed_files.contains(&file_struct_key) {
                continue;
            }
            processed_files.insert(file_struct_key);

            let content_lines: Vec<&str> = content.lines().collect();
            let struct_def_line = content_lines
                .iter()
                .position(|line| line.contains(&format!("struct {struct_name}")))
                .unwrap_or(0);

            let mut doc_comments = Vec::new();
            let mut line_idx = struct_def_line;

            while line_idx > 0 {
                line_idx -= 1;
                let line = content_lines[line_idx].trim();

                if line.starts_with("///") {
                    doc_comments.insert(0, line.trim_start_matches("///").trim());
                } else if !line.is_empty() && !line.starts_with('#') {
                    // If we found a non-empty line that is not a doc-comment or attribute,
                    // it means there are no more doc-comments
                    break;
                }
            }

            // Extract violation message
            let mut violation_message = String::new();
            let mut fix_mode = "Unfixable".to_string();

            // Look for message method implementation
            for idx in struct_def_line..content_lines.len() {
                let line = content_lines[idx].trim();
                if line.contains("fn message(&self)") {
                    // Find the return statement
                    for l in &content_lines[idx..] {
                        let l = l.trim();
                        if let Some(start_idx) = l.find('"') {
                            let mut in_escape = false;
                            let mut end_idx = None;
                            for (i, c) in l[start_idx + 1..].char_indices() {
                                match c {
                                    '\\' if !in_escape => in_escape = true,
                                    '"' if !in_escape => {
                                        end_idx = Some(i);
                                        break;
                                    }
                                    _ => in_escape = false,
                                }
                            }
                            if let Some(msg_end) = end_idx {
                                violation_message =
                                    l[start_idx + 1..start_idx + 1 + msg_end].to_string();
                                break;
                            }
                        }
                        if l.contains('}') {
                            break;
                        }
                    }
                }

                // Look for fix_mode implementation
                if line.contains("fn fix_mode(&self)") {
                    for l in &content_lines[idx..] {
                        let l = l.trim();
                        if l.contains("FixMode::Safe") {
                            fix_mode = "Safe".to_string();
                            break;
                        } else if l.contains("FixMode::Unsafe") {
                            fix_mode = "Unsafe".to_string();
                            break;
                        }
                        if l.contains('}') {
                            break;
                        }
                    }
                }

                // Break if we've found both
                if !violation_message.is_empty() && fix_mode != "Unfixable" {
                    break;
                }
            }

            if !doc_comments.is_empty() {
                let doc_content = doc_comments.join("\n");

                let md_dir = docs_dir.join(&group);
                create_dir_all(&md_dir).unwrap();

                let file_name = struct_name.chars().fold(String::new(), |mut acc, c| {
                    if c.is_uppercase() {
                        if !acc.is_empty() {
                            acc.push('-');
                        }
                        acc.push(c.to_ascii_lowercase());
                    } else {
                        acc.push(c);
                    }

                    acc
                });
                let rule_name = file_name.clone();
                let md_path = md_dir.join(format!("{file_name}.md"));
                let md_content = format!(
                    "# `{group}` `{rule_name}`\n\nSource: [{source_path}](../../{source_path})\n\n{doc_content}\n"
                );

                fs::write(&md_path, md_content).unwrap();

                rules_by_group.entry(group.clone()).or_default().push(RuleInfo {
                    rule_name,
                    file_name,
                    message: violation_message,
                    fix_mode,
                });
            }
        }
    }
}

fn generate_index_file(docs_dir: &Path, rules_by_group: &HashMap<String, Vec<RuleInfo>>) {
    let mut content = String::from("# Linter Rules\n\n");

    // Sort groups alphabetically
    let mut groups: Vec<&String> = rules_by_group.keys().collect();
    groups.sort();

    for group in groups {
        // Add group header
        writeln!(content, "## `{group}`\n").unwrap();

        // Add table header
        writeln!(content, "| Rule | Message | Fix Mode |").unwrap();
        writeln!(content, "|------|---------|----------|").unwrap();

        // Sort rules alphabetically within group
        let mut rules = rules_by_group[group].clone();
        rules.sort_by(|a, b| a.rule_name.cmp(&b.rule_name));

        for rule_info in rules {
            let rule_name = rule_info.rule_name;
            let file_name = rule_info.file_name;
            let message = rule_info.message.replace('{', "`{").replace('}', "}`");
            let fix_mode = rule_info.fix_mode;

            writeln!(
                content,
                "| [`{rule_name}`]({group}/{file_name}.md) | {message} | {fix_mode} |"
            )
            .unwrap();
        }

        content.push('\n');
    }

    fs::write(docs_dir.join("rules.md"), content).unwrap();
}
