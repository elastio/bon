#![allow(missing_docs)]

use anyhow::{bail, Context, Result};
use ra_ap_parser::{Edition, SyntaxKind, T};
use ra_ap_syntax::ast::edit_in_place::AttrsOwnerEdit;
use ra_ap_syntax::ast::HasAttrs;
use ra_ap_syntax::ted::Position;
use ra_ap_syntax::{ast, ted, AstNode, NodeOrToken, SourceFile};
use std::ffi::OsStr;
use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(err) = try_main() {
        eprintln!("{err:?}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn try_main() -> Result<()> {
    // Skip the executable path and collect CLI args
    let args: Vec<_> = std::env::args().skip(1).collect();
    let args: Vec<_> = args.iter().map(String::as_str).collect();

    match args.as_slice() {
        ["migrate"] => migrate(args.get(1).copied())?,
        _ => {
            bail!("Invalid command. Only `bon migrate` is supported");
        }
    }

    Ok(())
}

fn migrate(edition: Option<&str>) -> Result<()> {
    let edition = edition
        .map(str::parse)
        .transpose()?
        .unwrap_or(Edition::CURRENT);

    let rust_files = walkdir::WalkDir::new(".")
        .into_iter()
        .filter_entry(|entry| {
            let is_target_dir = entry.file_type().is_dir() && entry.file_name() == "target";
            !is_target_dir
        })
        .filter(|entry| {
            let Ok(entry) = entry else {
                return true;
            };
            entry.file_type().is_file() && entry.path().extension() == Some(OsStr::new("rs"))
        });

    for entry in rust_files {
        let entry = entry.context("Failed to read Rust source files")?;
        let path = entry.path();
        let file = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read the file '{}'", path.display()))?;

        eprintln!("Processing {}...", path.display());

        let file = migrate_rust_file(edition, &file)?;

        std::fs::write(path, file)
            .with_context(|| format!("Failed to write to the file '{}'", path.display()))?;
    }

    Ok(())
}

fn migrate_rust_file(edition: Edition, file: &str) -> Result<String> {
    // The `parse` method returns a `Parse` -- a pair of syntax tree and a list
    // of errors. That is, syntax tree is constructed even in presence of errors.
    let parse = SourceFile::parse(file, edition);

    let errors = parse.errors();

    if !errors.is_empty() {
        bail!("Encountered parsing errors: {errors:#?}");
    }

    let file = parse.tree().clone_for_update();

    let structs = file
        .syntax()
        .descendants()
        .filter_map(ast::Struct::cast)
        .collect::<Vec<_>>();

    for struct_item in structs {
        let builder_attr = struct_item.attrs().find_map(|attr| {
            let name_ref = attr.meta()?.path()?.segment()?.name_ref()?;
            (name_ref.text().as_str() == "builder").then_some(attr)
        });

        let Some(builder_attr) = builder_attr else {
            continue;
        };

        let derive_attr = struct_item
            .attrs()
            .find(|attr| attr.simple_name().as_deref() == Some("derive"));

        let is_empty_builder_attr = builder_attr
            .meta()
            .as_ref()
            .and_then(ast::Meta::token_tree)
            .map(|tt| tt.token_trees_and_tokens().next().is_none())
            .unwrap_or(true);

        if let Some(next_sibling) = builder_attr.syntax().next_sibling_or_token() {
            if next_sibling.kind() == SyntaxKind::WHITESPACE {
                ted::remove(next_sibling);
            }
        }

        ted::remove(builder_attr.syntax());

        if let Some(derive_attr) = derive_attr {
            if insert_builder_derive_into_existing(&derive_attr).is_some() {
                if !is_empty_builder_attr {
                    struct_item.add_attr(builder_attr);
                }
                continue;
            }
        }

        let bon_builder = ast::make::path_from_text("bon::Builder")
            .syntax()
            .descendants_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .map(NodeOrToken::Token);

        let tt = bon_builder.collect();

        let new_attr = ast::make::attr_outer(ast::make::meta_token_tree(
            ast::make::path_from_text("derive"),
            ast::make::token_tree(T!['('], tt),
        ))
        .clone_for_update();

        struct_item.add_attr(new_attr);
        if !is_empty_builder_attr {
            struct_item.add_attr(builder_attr);
        }
    }

    Ok(file.to_string())
}

fn insert_builder_derive_into_existing(derive_attr: &ast::Attr) -> Option<()> {
    let r_paren = derive_attr.meta()?.token_tree()?.r_paren_token()?;

    let has_trailing_comma = r_paren
        .prev_token()
        .is_some_and(|token| token.kind() == T!(,));

    let bon_builder = ast::make::path_from_text("bon::Builder")
        .syntax()
        .clone_for_update();

    let position = Position::before(r_paren);

    if has_trailing_comma {
        ted::insert(position, bon_builder);
    } else {
        ted::insert_all(
            position,
            vec![ast::make::token(T![,]).into(), bon_builder.into()],
        );
    }

    Some(())
}
