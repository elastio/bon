//! This script generates a module for each markdown file in the `website` directory
//! and attaches the content of the file as a doc attribute to the module so that
//! the we can test the rust example snippets in the website with `cargo test --doc`.

use std::path::PathBuf;
use walkdir::DirEntry;

fn main() {
    let website = "..";

    println!("cargo::rerun-if-changed={website}");

    let mut doc_tests = walkdir::WalkDir::new(website)
        .into_iter()
        .filter_entry(|entry| !is_hidden(entry))
        .map(|entry| {
            entry.unwrap_or_else(|err| {
                panic!("Failed to read dir entry in the `website` directory: {err:#?}")
            })
        })
        .filter(|entry| {
            entry.file_type().is_file()
                && entry.path().extension() == Some("md".as_ref())
                && !entry.path().starts_with("../src/v1")
                && !entry.path().starts_with("../src/v2")
        })
        .map(DirEntry::into_path)
        .collect::<Vec<_>>();

    doc_tests.sort_unstable();

    let doc_tests = doc_tests
        .into_iter()
        .map(|path| {
            let test_name = path
                .iter()
                .skip(1)
                .map(|component| {
                    component
                        .to_str()
                        .unwrap()
                        .replace(['.', '-'], "_")
                        .to_lowercase()
                })
                .collect::<Vec<_>>()
                .join("_");

            let doc = std::fs::read_to_string(&path).unwrap();

            // Remove VitePress-specific `[attrs...]` from code blocks because rustdoc
            // doesn't understand them and skips the doc tests.
            let doc = doc
                .lines()
                .map(|line| {
                    if !line.contains("```") {
                        return line.to_owned();
                    }

                    let mut in_brackets = false;

                    line.chars()
                        .filter(|&c| match c {
                            '[' => {
                                in_brackets = true;
                                false
                            }
                            ']' => {
                                in_brackets = false;
                                false
                            }
                            _ => !in_brackets,
                        })
                        .collect()
                })
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                "#[doc = r#####\"{doc}\"#####] \
                mod {test_name} {{}}"
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_file = PathBuf::from_iter([&out_dir, "website_doctests.rs"]);

    std::fs::write(out_file, doc_tests).unwrap();
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| s == "node_modules" || (s != ".." && s.starts_with('.')))
}
