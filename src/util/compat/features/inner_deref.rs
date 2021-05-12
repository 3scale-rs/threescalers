#[cfg(not(has_core_result_Result_as_deref))]
pub trait InnerDerefExt<T, E>
where
    T: core::ops::Deref,
{
    fn as_deref(&self) -> Result<&T::Target, &E>;
}

#[cfg(not(has_core_result_Result_as_deref))]
impl<T: core::ops::Deref, E> InnerDerefExt<T, E> for Result<T, E> {
    fn as_deref(&self) -> Result<&T::Target, &E> {
        self.as_ref().map(|t| t.deref())
    }
}

#[cfg(not(has_core_result_Result_as_deref_mut))]
pub trait InnerDerefMutExt<T, E>
where
    T: core::ops::DerefMut,
{
    fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E>;
}

#[cfg(not(has_core_result_Result_as_deref_mut))]
impl<T: core::ops::DerefMut, E> InnerDerefMutExt<T, E> for Result<T, E> {
    fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E> {
        self.as_mut().map(|t| t.deref_mut())
    }
}
