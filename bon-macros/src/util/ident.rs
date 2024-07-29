use easy_ext::ext;
use heck::ToPascalCase;
use proc_macro2::Span;

#[ext(IdentExt)]
pub(crate) impl syn::Ident {
    /// Converts the ident to PascalCase without preserving its span.
    ///
    /// Span loss is intentional to work around the semantic token type assignment
    /// ambiguity that may be experienced by IDEs. For example, rust analyzer
    /// assigns different colors to identifiers according to their semantic meaning.
    ///
    /// If identifiers with the same span were used in different contexts such as
    /// in function name and struct name positions, then rust-analyzer would chose
    /// the semantic meaning for syntax highlighting of the input identifier randomly
    /// out of these two contexts.
    ///
    /// By not preserving the span, we can ensure that the semantic meaning of the
    /// produced identifier won't influence the syntax highlighting of the original
    /// identifier.
    fn to_pascal_case(&self) -> Self {
        // There are no pascal case keywords in Rust except for `Self`, which
        // is anyway not allowed even as a raw identifier:
        // https://internals.rust-lang.org/t/raw-identifiers-dont-work-for-all-identifiers/9094
        //
        // So no need to handle raw identifiers here.
        syn::Ident::new(&self.raw_name().to_pascal_case(), Span::call_site())
    }

    /// Creates a new ident with the given name and span. If the name starts with
    /// `r#` then automatically creates a raw ident.
    fn new_maybe_raw(name: &str, span: Span) -> Self {
        if let Some(name) = name.strip_prefix("r#") {
            syn::Ident::new_raw(name, span)
        } else {
            syn::Ident::new(name, span)
        }
    }

    /// Returns the name of the identifier stripping the `r#` prefix if it exists.
    fn raw_name(&self) -> String {
        let name = self.to_string();
        if let Some(raw) = name.strip_prefix("r#") {
            raw.to_owned()
        } else {
            name
        }
    }
}
