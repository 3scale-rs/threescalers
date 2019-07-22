// make the err_val macro available to the crate
#![macro_use]

// re-publishing here for ease of use of modules using this
#[allow(unused_imports)]
pub(crate) use snafu::{
    ensure,
    Backtrace,
    OptionExt,
    ResultExt,
    Snafu,
};

// This macro is meant to generate the Error value of a snafu error for usage in
// contexts where the actual snafu error variant value is needed, such as in
// Option#ok_or_else()?. You just pass in the snafu struct representing the error
// variant and get back the enum type with a value matching that error variant.
//
// Note that the snafu's fail() fn receives a type parameter for the Result type
// it must generate, but since it will always be an error and we just unwrap it
// right away, we should not need to concern ourselves with it and can tell the
// compiler that a Result with the Ok variant will never happen.
macro_rules! err_val {
    ($e:expr) => {
        $e.fail::<crate::Never>().unwrap_err()
    };
}

// A general API error type.
//
// The snafu crate mostly prods you to add local error types in modules, which is
// a good design strategy. However, some of those might end up eventually translated
// into a more general error, and this would be it. The decision on doing that has
// not yet been taken, so we might end up declaring several local error types as part
// of the public API contract. Currently this serves as a fallback for those places
// where we don't yet have a local error type defined.
//
// Note: the non-exhaustiveness is meant for public API enums to force users to
// be forward-compatible when we add extra errors to these enums. Stable rust does
// not currently provide a proper way to do this, so a __Nonexhaustive variant is
// maintained at the end of this enum. Nightly already has a proper way to handle
// this via a feature flag.
//
// Note: before we commit to an error API, we should look into making this an opaque
// type. The snafu crate has some documentation around this we can use to consider how
// to best make this a part of the API.
#[derive(Debug, Snafu)]
#[cfg_attr(feature = "nightly", non_exhaustive)]
pub enum ThreescalersError {
    // So far we don't have specific errors here.
    #[snafu(display("Unknown threescalers error: {}", msg))]
    Unknown { msg: String },
    #[cfg(not(feature = "nightly"))]
    #[doc(hidden)]
    __Nonexhaustive,
}

// A general Result type that can yield a general API error
pub type Result<T, E = ThreescalersError> = std::result::Result<T, E>;
