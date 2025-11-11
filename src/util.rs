#[macro_use]
pub mod compat;
#[cfg(feature = "http-types")]
pub use compat::features::Never;

pub mod string;
