#[must_use]
pub fn convert(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
