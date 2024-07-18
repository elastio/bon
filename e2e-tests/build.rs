use heck::ToSnakeCase;
use itertools::Itertools;
use std::path::PathBuf;
use walkdir::DirEntry;

fn main() {
    let doc_tests = walkdir::WalkDir::new("../website")
        .into_iter()
        .filter_entry(|entry| !is_hidden(entry))
        .map(|entry| {
            entry.unwrap_or_else(|err| {
                panic!("Failed to read dir entry in the `website` directory: {err:#?}")
            })
        })
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension() == Some("md".as_ref())
        })
        .map(|entry| entry.into_path().into_os_string().into_string().unwrap())
        .sorted_unstable()
        .map(|path| {
            println!("cargo::rerun-if-changed={path}");

            let test_name = path.to_snake_case();
            let test_name = test_name.strip_prefix("website_").unwrap();

            let canonical_path = std::fs::canonicalize(path).unwrap();
            let canonical_path = canonical_path.to_string_lossy();

            format!(
                "#[doc = include_str!(\"{canonical_path}\")] \
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
        .is_some_and(|s| s == "node_modules" || s.starts_with('.'))
}
