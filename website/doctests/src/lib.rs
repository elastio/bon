//! Crate for testing Rust code snippets in markdown files on the website.

#[cfg(doctest)]
// We use a bunch of Vitepress-specific syntax in the doctests, for example to
// give a name to a code group in a fenced code block, which conflicts with this
// lint.
#[deny(rustdoc::invalid_codeblock_attributes)]
mod website_doctests {
    include!(concat!(env!("OUT_DIR"), "/website_doctests.rs"));
}
