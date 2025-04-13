#[cfg(supports_never_type)]
pub type Never = !;
#[cfg(not(supports_never_type))]
pub type Never = core::convert::Infallible;
