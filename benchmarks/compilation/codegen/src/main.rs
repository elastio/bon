use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let bench_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?)
        .parent()
        .unwrap()
        .to_path_buf();

    let src_dir = bench_dir.join("src");

    let structs_number = 200;
    let fields_number = 20;

    std::fs::write(
        src_dir.join(format!(
            "structs_{structs_number}_fields_{fields_number}.rs"
        )),
        structs_n_fields_n(structs_number, fields_number).to_string(),
    )?;

    println!("Running cargo fmt in {}", bench_dir.display());

    let status = std::process::Command::new("cargo")
        .arg("fmt")
        .current_dir(&bench_dir)
        .spawn()?
        .wait()?;

    anyhow::ensure!(status.success(), "cargo fmt failed: {status}");

    Ok(())
}

fn structs_n_fields_n(structs_number: usize, fields_number: usize) -> TokenStream {
    let field_names = (1..=fields_number)
        .map(|i| format_ident!("x{i}"))
        .collect::<Vec<_>>();

    (1..=structs_number)
        .map(move |i| {
            let struct_name = format_ident!("Struct{i}");

            quote! {
                #[cfg_attr(
                    any(
                        feature = "bon",
                        feature = "typed-builder",
                        feature = "derive_builder",
                    ),
                    derive(crate::Builder),
                )]
                #[cfg_attr(
                    feature = "bon-overwritable",
                    builder(on(_, overwritable)),
                )]
                pub struct #struct_name {
                    #( #field_names: i32, )*
                }
            }
        })
        .collect()
}
