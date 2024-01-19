use super::{for_both, Either, Left, Right};
use core::iter;

impl<L, R> Either<L, R> {
    /// Convert the inner value to an iterator.
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, Vec<u32>> = Left(vec![1, 2, 3, 4, 5]);
    /// let mut right: Either<Vec<u32>, _> = Right(vec![]);
    /// right.extend(left.into_iter());
    /// assert_eq!(right, Right(vec![1, 2, 3, 4, 5]));
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> Either<L::IntoIter, R::IntoIter>
    where
        L: IntoIterator,
        R: IntoIterator<Item = L::Item>,
    {
        match self {
            Left(l) => Left(l.into_iter()),
            Right(r) => Right(r.into_iter()),
        }
    }

    /// Borrow the inner value as an iterator.
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, &[u32]> = Left(vec![2, 3]);
    /// let mut right: Either<Vec<u32>, _> = Right([4, 5].as_slice());
    /// let mut all = vec![1];
    /// all.extend(left.iter());
    /// all.extend(right.iter());
    /// assert_eq!(all, vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn iter(&self) -> Either<<&L as IntoIterator>::IntoIter, <&R as IntoIterator>::IntoIter>
    where
        for<'a> &'a L: IntoIterator,
        for<'a> &'a R: IntoIterator<Item = <&'a L as IntoIterator>::Item>,
    {
        self.as_ref().into_iter()
    }

    /// Mutably borrow the inner value as an iterator.
    ///
    /// ```
    /// use either::*;
    ///
    /// let mut left: Either<_, &mut [u32]> = Left(vec![2, 3]);
    /// for l in left.iter_mut() {
    ///     *l *= *l
    /// }
    /// assert_eq!(left, Left(vec![4, 9]));
    ///
    /// let mut inner = [4, 5];
    /// let mut right: Either<Vec<u32>, _> = Right(inner.as_mut_slice());
    /// for r in right.iter_mut() {
    ///     *r *= *r
    /// }
    /// assert_eq!(inner, [16, 25]);
    /// ```
    pub fn iter_mut(
        &mut self,
    ) -> Either<<&mut L as IntoIterator>::IntoIter, <&mut R as IntoIterator>::IntoIter>
    where
        for<'a> &'a mut L: IntoIterator,
        for<'a> &'a mut R: IntoIterator<Item = <&'a mut L as IntoIterator>::Item>,
    {
        self.as_mut().into_iter()
    }

    /// Factors an `Either` of `Iterator`s to be an `Iterator` of `Either`s
    /// ```
    /// use either::*;
    /// let left: Either<_, Vec<u8>> = Left(&["hello"]);
    /// assert_eq!(left.factor_iter().next(), Some(Left(&"hello")));

    /// let right: Either<&[&str], _> = Right(vec![0, 1]);
    /// assert_eq!(right.factor_iter().collect::<Vec<_>>(), vec![Right(0), Right(1)]);
    ///
    /// ```
    // TODO(MSRV): doc(alias) was stabilized in Rust 1.48
    // #[doc(alias = "transpose")]
    pub fn factor_iter(
        self,
    ) -> Either<
        core::iter::Map<L::IntoIter, impl Fn(L::Item) -> Either<L::Item, R::Item>>,
        core::iter::Map<R::IntoIter, impl Fn(R::Item) -> Either<L::Item, R::Item>>,
    >
    where
        L: IntoIterator,
        R: IntoIterator,
    {
        self.map_either(
            |l| l.into_iter().map(Either::Left),
            |r| r.into_iter().map(Either::Right),
        )
    }
}

impl<L, R, A> Extend<A> for Either<L, R>
where
    L: Extend<A>,
    R: Extend<A>,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A>,
    {
        for_both!(*self, ref mut inner => inner.extend(iter))
    }
}

/// `Either<L, R>` is an iterator if both `L` and `R` are iterators.
impl<L, R> Iterator for Either<L, R>
where
    L: Iterator,
    R: Iterator<Item = L::Item>,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for_both!(*self, ref mut inner => inner.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        for_both!(*self, ref inner => inner.size_hint())
    }

    fn fold<Acc, G>(self, init: Acc, f: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        for_both!(self, inner => inner.fold(init, f))
    }

    fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item),
    {
        for_both!(self, inner => inner.for_each(f))
    }

    fn count(self) -> usize {
        for_both!(self, inner => inner.count())
    }

    fn last(self) -> Option<Self::Item> {
        for_both!(self, inner => inner.last())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        for_both!(*self, ref mut inner => inner.nth(n))
    }

    fn collect<B>(self) -> B
    where
        B: iter::FromIterator<Self::Item>,
    {
        for_both!(self, inner => inner.collect())
    }

    fn partition<B, F>(self, f: F) -> (B, B)
    where
        B: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> bool,
    {
        for_both!(self, inner => inner.partition(f))
    }

    fn all<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        for_both!(*self, ref mut inner => inner.all(f))
    }

    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        for_both!(*self, ref mut inner => inner.any(f))
    }

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        for_both!(*self, ref mut inner => inner.find(predicate))
    }

    fn find_map<B, F>(&mut self, f: F) -> Option<B>
    where
        F: FnMut(Self::Item) -> Option<B>,
    {
        for_both!(*self, ref mut inner => inner.find_map(f))
    }

    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        for_both!(*self, ref mut inner => inner.position(predicate))
    }
}

impl<L, R> DoubleEndedIterator for Either<L, R>
where
    L: DoubleEndedIterator,
    R: DoubleEndedIterator<Item = L::Item>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        for_both!(*self, ref mut inner => inner.next_back())
    }

    // TODO(MSRV): This was stabilized in Rust 1.37
    // fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
    //     for_both!(*self, ref mut inner => inner.nth_back(n))
    // }

    fn rfold<Acc, G>(self, init: Acc, f: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        for_both!(self, inner => inner.rfold(init, f))
    }

    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        for_both!(*self, ref mut inner => inner.rfind(predicate))
    }
}

impl<L, R> ExactSizeIterator for Either<L, R>
where
    L: ExactSizeIterator,
    R: ExactSizeIterator<Item = L::Item>,
{
    fn len(&self) -> usize {
        for_both!(*self, ref inner => inner.len())
    }
}

impl<L, R> iter::FusedIterator for Either<L, R>
where
    L: iter::FusedIterator,
    R: iter::FusedIterator<Item = L::Item>,
{
}