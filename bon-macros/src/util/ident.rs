use easy_ext::ext;
use heck::ToPascalCase;

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
        quote::format_ident!("{}", self.to_string().to_pascal_case())
    }
}
