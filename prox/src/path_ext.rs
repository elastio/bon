use easy_ext::ext;

#[ext(PathExt)]
pub impl syn::Path {
    /// Check if the path ends with the given segment.
    fn ends_with_segment(&self, desired_segment: &str) -> bool {
        self.segments
            .last()
            .is_some_and(|last| last.ident == desired_segment)
    }
}
