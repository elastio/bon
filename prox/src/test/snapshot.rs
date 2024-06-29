use crate::Result;
use expect_test::Expect;
use proc_macro2::TokenStream as TokenStream2;

/// Assert that the given proc macro generation function produces the expected output
/// according to the [`Expect`] snapshot.
///
// FIXME: combine this test with `trybuild` test.
// E.g. feed the input code from the same file both to `trybuild` and
// to macro expansion function?
#[track_caller]
pub fn assert_gen_attr_fn_macro<Opts>(
    sut: fn(Opts, syn::ItemFn) -> Result<TokenStream2>,
    input_code: TokenStream2,
    expected: Expect,
) where
    Opts: darling::FromMeta,
{
    let mut item: syn::ItemFn =
        syn::parse2(input_code).expect("BUG: input Rust code could not be parsed");

    // We expect the tested macro to be the top macro in the code
    let attr = item.attrs.remove(0);
    let meta = match attr.meta {
        syn::Meta::Path(_) => Default::default(),
        syn::Meta::List(list) => list.tokens,
        syn::Meta::NameValue(_) => panic!("Unexpected name-value attribute"),
    };

    let meta = darling::ast::NestedMeta::parse_meta_list(meta.clone()).unwrap_or_else(|err| {
        panic!(
            "Failed to parse macro options as NestedMeta.\n\
                Error: {err:#?}\n\
                Meta: {meta:#?}",
        )
    });

    let opts = Opts::from_list(&meta).unwrap_or_else(|err| {
        panic!(
            "Failed to parse macro options.\n\
            Error: {err:#?}\n\
            Meta: {meta:#?}",
        )
    });

    let output = sut(opts, item).expect("test macro failed");

    expected.assert_eq(&rustfmt(&output.to_string()));
}

fn rustfmt(code: &str) -> String {
    devx_cmd::cmd!("rustfmt")
        .stdin(code)
        .arg2("--edition", "2021")
        .read()
        .expect("Formatting Rust code failed")
}
