#[doc(hidden)]
#[deprecated(note = "\
    #[tracing::instrument] attribute should be placed before the #[builder] attribute \
    to make sure it uses the original function name as the span name;\n\n\
    reason: when the #[tracing::instrument] attribute is placed after the #[builder] \
    attribute it will run after the #[builder] attribute is expanded, at which point \
    the original function will be renamed to __orig_{fn_name}, and this will make \
    that extra __orig_ prefix appear in the span name, which is likely not what you want")]
pub mod tracing_instrument_attribute_after_builder {}
