use std::io::{self, Write, Read, BufRead};

pub use Either::{Left, Right};

/// `Either` represents an alternative holding one value out of
/// either of the two possible values.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

macro_rules! either {
    ($value:expr, $inner:ident => $result:expr) => (
        match $value {
            Either::Left(ref $inner) => $result,
            Either::Right(ref $inner) => $result,
        }
    )
}

macro_rules! either_mut {
    ($value:expr, $inner:ident => $result:expr) => (
        match $value {
            Either::Left(ref mut $inner) => $result,
            Either::Right(ref mut $inner) => $result,
        }
    )
}

impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {
        match *self {
            Left(_) => true,
            Right(_) => false,
        }
    }

    pub fn is_right(&self) -> bool {
        !self.is_left()
    }

    pub fn left(self) -> Option<L> {
        match self {
            Left(l) => Some(l),
            Right(_) => None,
        }
    }

    pub fn right(self) -> Option<R> {
        match self {
            Left(_) => None,
            Right(r) => Some(r),
        }
    }

    pub fn as_ref(&self) -> Either<&L, &R> {
        match *self {
            Left(ref inner) => Left(inner),
            Right(ref inner) => Right(inner),
        }
    }

    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match *self {
            Left(ref mut inner) => Left(inner),
            Right(ref mut inner) => Right(inner),
        }
    }
}

/// Convert from `Result` to `Either` with `Ok => Right` and `Err => Left`.
impl<L, R> From<Result<R, L>> for Either<L, R> {
    fn from(r: Result<R, L>) -> Self {
        match r {
            Err(e) => Left(e),
            Ok(o) => Right(o),
        }
    }
}

/// Convert from `Either` to `Result` with `Right => Ok` and `Left => Err`.
impl<L, R> Into<Result<R, L>> for Either<L, R> {
    fn into(self) -> Result<R, L> {
        match self {
            Left(l) => Err(l),
            Right(r) => Ok(r),
        }
    }
}

impl<L, R> Iterator for Either<L, R>
    where L: Iterator, R: Iterator<Item=L::Item>
{
    type Item = L::Item;

    fn next(&mut self) -> Option<L::Item> {
        either_mut!(*self, inner => inner.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        either!(*self, inner => inner.size_hint())
    }
}

impl<L, R> DoubleEndedIterator for Either<L, R>
    where L: DoubleEndedIterator, R: DoubleEndedIterator<Item=L::Item>
{
    fn next_back(&mut self) -> Option<L::Item> {
        either_mut!(*self, inner => inner.next_back())
    }
}

impl<L, R> Read for Either<L, R>
    where L: Read, R: Read
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        either_mut!(*self, inner => inner.read(buf))
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        either_mut!(*self, inner => inner.read_to_end(buf))
    }
}

impl<L, R> BufRead for Either<L, R>
    where L: BufRead, R: BufRead
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        either_mut!(*self, inner => inner.fill_buf())
    }

    fn consume(&mut self, amt: usize) {
        either_mut!(*self, inner => inner.consume(amt))
    }
}

impl<L, R> Write for Either<L, R>
    where L: Write, R: Write
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        either_mut!(*self, inner => inner.write(buf))
    }

    fn flush(&mut self) -> io::Result<()> {
        either_mut!(*self, inner => inner.flush())
    }
}

#[test]
fn it_works() {
}
