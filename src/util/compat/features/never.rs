// Depending on features used, this type might become unused
#[cfg(supports_never_type)]
#[allow(dead_code)]
pub type Never = !;
#[cfg(not(supports_never_type))]
#[allow(dead_code)]
pub type Never = core::convert::Infallible;
