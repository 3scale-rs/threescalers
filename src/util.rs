#[macro_use]
pub mod compat;
#[cfg(feature = "never_type")]
pub use compat::features::Never;

pub mod string;
