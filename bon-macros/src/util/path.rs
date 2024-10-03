pub(crate) trait PathExt {
    /// Check if the path starts with the given segment.
    fn starts_with_segment(&self, desired_segment: &str) -> bool;

    /// Check if the path ends with the given segment.
    fn ends_with_segment(&self, desired_segment: &str) -> bool;
}

impl PathExt for syn::Path {
    fn starts_with_segment(&self, desired_segment: &str) -> bool {
        self.segments
            .first()
            .map(|first| first.ident == desired_segment)
            .unwrap_or(false)
    }

    fn ends_with_segment(&self, desired_segment: &str) -> bool {
        self.segments
            .last()
            .map(|last| last.ident == desired_segment)
            .unwrap_or(false)
    }
}
