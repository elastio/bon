use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let bench_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?)
        .parent()
        .unwrap()
        .to_path_buf();

    let src_dir = bench_dir.join("src");

    for (structs_number, fields_number) in [(100, 10), (10, 50)] {
        std::fs::write(
            src_dir.join(format!(
                "structs_{structs_number}_fields_{fields_number}.rs"
            )),
            structs_n_fields_n(structs_number, fields_number).to_string(),
        )?;
    }

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

    let field_values = (1..=fields_number)
        .map(Literal::usize_unsuffixed)
        .collect::<Vec<_>>();

    (1..=structs_number)
        .map(move |i| {
            let struct_name = format_ident!("Struct{i}");
            let builder_name = format_ident!("Struct{i}Builder");
            let usage_fn = format_ident!("struct{i}");

            // Every struct is built once with all setters called. This way
            // the benchmark measures the cost of the builder usage in addition
            // to the cost of the builder definition. The `cfg`s follow the
            // same order of priority as the `Builder` import in `lib.rs`.
            quote! {
                #[cfg_attr(
                    any(
                        feature = "bon",
                        feature = "typed-builder",
                        feature = "derive_builder",
                    ),
                    derive(crate::Builder),
                )]
                pub struct #struct_name {
                    #( #field_names: i32, )*
                }

                #[cfg(any(feature = "bon", feature = "typed-builder"))]
                pub fn #usage_fn() -> #struct_name {
                    #struct_name::builder()
                        #( .#field_names(#field_values) )*
                        .build()
                }

                #[cfg(all(
                    feature = "derive_builder",
                    not(any(feature = "bon", feature = "typed-builder")),
                ))]
                pub fn #usage_fn() -> #struct_name {
                    #builder_name::default()
                        #( .#field_names(#field_values) )*
                        .build()
                        .unwrap()
                }

                #[cfg(not(any(
                    feature = "bon",
                    feature = "typed-builder",
                    feature = "derive_builder",
                )))]
                pub fn #usage_fn() -> #struct_name {
                    #struct_name {
                        #( #field_names: #field_values, )*
                    }
                }
            }
        })
        .collect()
}
