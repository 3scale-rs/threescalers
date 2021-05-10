#[cfg(not(supports_inner_deref))]
mod inner_deref;
#[cfg(not(supports_inner_deref))]
pub use inner_deref::{InnerDerefExt, InnerDerefMutExt};

#[cfg(not(has_core_iter_Iterator_reduce))]
mod iterator_fold_self;
#[cfg(not(has_core_iter_Iterator_reduce))]
pub use iterator_fold_self::IteratorFoldSelfExt;
