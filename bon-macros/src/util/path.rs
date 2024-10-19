pub(crate) trait PathExt {
    /// Check if the path starts with the given segment.
    fn starts_with_segment(&self, desired_segment: &str) -> bool;
}

impl PathExt for syn::Path {
    fn starts_with_segment(&self, desired_segment: &str) -> bool {
        self.segments
            .first()
            .map(|first| first.ident == desired_segment)
            .unwrap_or(false)
    }
}
