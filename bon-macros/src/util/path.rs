use easy_ext::ext;

#[ext(PathExt)]
pub impl syn::Path {
    /// Check if the path ends with the given segment.
    fn ends_with_segment(&self, desired_segment: &str) -> bool {
        self.segments
            .last()
            .is_some_and(|last| last.ident == desired_segment)
    }

    /// Check if the path starts with the given segment.
    fn starts_with_segment(&self, desired_segment: &str) -> bool {
        self.segments
            .first()
            .is_some_and(|first| first.ident == desired_segment)
    }
}
