pub trait IteratorFoldSelfExt: core::iter::Iterator {
    // this feature used to have fold_first as well,
    // but since the impl is so simple, we just add
    // our own reduce even if fold_first would be
    // available via a feature gate.
    fn reduce<F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Self::Item;
}

impl<I> IteratorFoldSelfExt for I
where
    I: core::iter::Iterator,
{
    fn reduce<F>(mut self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        let first = self.next()?;
        Some(self.fold(first, f))
    }
}
