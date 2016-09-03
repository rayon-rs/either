//! The enum `Either`.

use std::error::Error;
use std::fmt;
use std::io::{self, Write, Read, BufRead};
use std::convert::{AsRef, AsMut};
use std::ops::Deref;
use std::ops::DerefMut;

pub use Either::{Left, Right};

/// `Either` represents an alternative holding one value out of
/// either of the two possible values.
///
/// `Either` is a general purpose sum type of two parts. For representing
/// success or error, use the regular `Result<T, E>` instead.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Either<L, R> {
    /// A value of type `L`.
    Left(L),
    /// A value of type `R`.
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

/// Macro for unwrapping the left side of an `Either`, which fails early
/// with the opposite side. Can only be used in functions that return
/// `Either` because of the early return of `Right` that it provides.
///
/// See also `try_right!` for its dual, which applies the same just to the
/// right side.
///
/// # Example
///
/// ```
/// #[macro_use] extern crate either;
/// use either::{Either, Left, Right};
///
/// fn twice(wrapper: Either<u32, &str>) -> Either<u32, &str> {
///     let value = try_left!(wrapper);
///     Left(value * 2)
/// }
///
/// fn main() {
///     assert_eq!(twice(Left(2)), Left(4));
///     assert_eq!(twice(Right("ups")), Right("ups"));
/// }
/// ```
#[macro_export]
macro_rules! try_left {
    ($expr:expr) => (
        match $expr {
            $crate::Left(val) => val,
            $crate::Right(err) => return $crate::Right(::std::convert::From::from(err))
        }
    )
}

/// Dual to `try_left!`, see its documentation for more information.
#[macro_export]
macro_rules! try_right {
    ($expr:expr) => (
        match $expr {
            $crate::Left(err) => return $crate::Left(::std::convert::From::from(err)),
            $crate::Right(val) => val
        }
    )
}

impl<L, R> Either<L, R> {
    /// Returns true if the value is `Left(L)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, ()> = Left(123);
    /// assert_eq!(left.is_left(), true);
    ///
    /// let right: Either<(), _> = Right("the right value");
    /// assert_eq!(right.is_left(), false);
    /// ```
    pub fn is_left(&self) -> bool {
        match *self {
            Left(_) => true,
            Right(_) => false,
        }
    }

    /// Returns true if the value is `Right(R)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, ()> = Left(123);
    /// assert_eq!(left.is_right(), false);
    ///
    /// let right: Either<(), _> = Right("the right value");
    /// assert_eq!(right.is_right(), true);
    /// ```
    pub fn is_right(&self) -> bool {
        !self.is_left()
    }

    /// Converts the left side of `Either<L, R>` to an `Option<L>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, ()> = Left("some value");
    /// assert_eq!(left.left(),  Some("some value"));
    ///
    /// let right: Either<(), _> = Right(321);
    /// assert_eq!(right.left(), None);
    /// ```
    pub fn left(self) -> Option<L> {
        match self {
            Left(l) => Some(l),
            Right(_) => None,
        }
    }

    /// Converts the right side of `Either<L, R>` to an `Option<R>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, ()> = Left("some value");
    /// assert_eq!(left.right(),  None);
    ///
    /// let right: Either<(), _> = Right(321);
    /// assert_eq!(right.right(), Some(321));
    /// ```
    pub fn right(self) -> Option<R> {
        match self {
            Left(_) => None,
            Right(r) => Some(r),
        }
    }

    /// Converts `Either<L, R>` to `Either<&L, &R>`.
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, ()> = Left("some value");
    /// assert_eq!(left.as_ref(), Left(&"some value"));
    ///
    /// let right: Either<(), _> = Right("some value");
    /// assert_eq!(right.as_ref(), Right(&"some value"));
    /// ```
    pub fn as_ref(&self) -> Either<&L, &R> {
        match *self {
            Left(ref inner) => Left(inner),
            Right(ref inner) => Right(inner),
        }
    }

    /// Converts `Either<L, R>` to `Either<&mut L, &mut R>`.
    ///
    /// ```
    /// use either::*;
    ///
    /// fn mutate(r: &mut Either<u32, u32>) {
    ///     match r.as_mut() {
    ///         Left(&mut ref mut l) => *l = 314,
    ///         Right(&mut ref mut r) => *r = 1,
    ///     }
    /// }
    ///
    /// let mut left = Left(123);
    /// mutate(&mut left);
    /// assert_eq!(left, Left(314));
    ///
    /// let mut right = Right(123);
    /// mutate(&mut right);
    /// assert_eq!(right, Right(1));
    /// ```
    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match *self {
            Left(ref mut inner) => Left(inner),
            Right(ref mut inner) => Right(inner),
        }
    }

    /// Converts `Either<L, R>` to `Either<R, L>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, ()> = Left(123);
    /// assert_eq!(left.flip(), Right(123));
    ///
    /// let right: Either<(), _> = Right("some value");
    /// assert_eq!(right.flip(), Left("some value"));
    /// ```
    pub fn flip(self) -> Either<R, L> {
        match self {
            Left(l) => Right(l),
            Right(r) => Left(r),
        }
    }

    /// Applies the function `f` on the `Left(L)` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, u32> = Left(123);
    /// assert_eq!(left.map_left(|x| x * 2), Left(246));
    ///
    /// let right: Either<u32, _> = Right(123);
    /// assert_eq!(right.map_left(|x| x * 2), Right(123));
    /// ```
    pub fn map_left<F, M>(self, f: F) -> Either<M, R>
        where F: FnOnce(L) -> M {
        match self {
            Left(l) => Left(f(l)),
            Right(r) => Right(r),
        }
    }

    /// Applies the function `f` on the `Right(R)` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use either::*;
    ///
    /// let left: Either<_, u32> = Left(123);
    /// assert_eq!(left.map_right(|x| x * 2), Left(123));
    ///
    /// let right: Either<u32, _> = Right(123);
    /// assert_eq!(right.map_right(|x| x * 2), Right(246));
    /// ```
    pub fn map_right<F, S>(self, f: F) -> Either<L, S>
        where F: FnOnce(R) -> S {
        match self {
            Left(l) => Left(l),
            Right(r) => Right(f(r)),
        }
    }

    /// Applies one of two functions depending on contents, unifying their result. If the value is
    /// `Left(L)` then the first function `f` is applied; if it is `Right(R)` then the second
    /// function `g` is applied.
    ///
    /// # Examples
    ///
    /// ```
    /// use either::*;
    ///
    /// fn square(n: u32) -> i32 { (n * n) as i32 }
    /// fn negate(n: i32) -> i32 { -n }
    ///
    /// let left: Either<u32, i32> = Left(4);
    /// assert_eq!(left.either(square, negate), 16);
    ///
    /// let right: Either<u32, i32> = Right(-4);
    /// assert_eq!(right.either(square, negate), 4);
    /// ```
    pub fn either<F, G, T>(self, f: F, g: G) -> T
      where F: FnOnce(L) -> T,
            G: FnOnce(R) -> T {
        match self {
            Left(l) => f(l),
            Right(r) => g(r),
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

/// `Either<L, R>` is an iterator if both `L` and `R` are iterators.
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

impl<L, R> ExactSizeIterator for Either<L, R>
    where L: ExactSizeIterator, R: ExactSizeIterator<Item=L::Item>
{
}

/// `Either<L, R>` implements `Read` if both `L` and `R` do.
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

/// `Either<L, R>` implements `Write` if both `L` and `R` do.
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

impl<L, R, Target> AsRef<Target> for Either<L, R>
    where L: AsRef<Target>, R: AsRef<Target>
{
    fn as_ref(&self) -> &Target {
        either!(*self, inner => inner.as_ref())
    }
}

impl<L, R, Target> AsMut<Target> for Either<L, R>
    where L: AsMut<Target>, R: AsMut<Target>
{
    fn as_mut(&mut self) -> &mut Target {
        either_mut!(*self, inner => inner.as_mut())
    }
}

impl<L, R> Deref for Either<L, R>
    where L: Deref, R: Deref<Target=L::Target>
{
    type Target = L::Target;

    fn deref(&self) -> &Self::Target {
        either!(*self, inner => &*inner)
    }
}

impl<L, R> DerefMut for Either<L, R>
    where L: DerefMut, R: DerefMut<Target=L::Target>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        either_mut!(*self, inner => &mut *inner)
    }
}

/// `Either` implements `Error` if *both* `L` and `R` implement it.
impl<L, R> Error for Either<L, R>
    where L: Error, R: Error
{
    fn description(&self) -> &str {
        either!(*self, inner => inner.description())
    }

    fn cause(&self) -> Option<&Error> {
        either!(*self, inner => inner.cause())
    }
}

impl<L, R> fmt::Display for Either<L, R>
    where L: fmt::Display, R: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        either!(*self, inner => inner.fmt(f))
    }
}

#[test]
fn basic() {
    let mut e = Left(2);
    let r = Right(2);
    assert_eq!(e, Left(2));
    e = r;
    assert_eq!(e, Right(2));
    assert_eq!(e.left(), None);
    assert_eq!(e.right(), Some(2));
    assert_eq!(e.as_ref().right(), Some(&2));
    assert_eq!(e.as_mut().right(), Some(&mut 2));
}

#[test]
fn macros() {
    fn a() -> Either<u32, u32> {
        let x: u32 = try_left!(Right(1337u32));
        Left(x * 2)
    }
    assert_eq!(a(), Right(1337));

    fn b() -> Either<String, &'static str> {
        Right(try_right!(Left("foo bar")))
    }
    assert_eq!(b(), Left(String::from("foo bar")));
}

#[test]
fn deref() {
    fn is_str(_: &str) {}
    let value: Either<String, &str> = Left(String::from("test"));
    is_str(&*value);
}

#[test]
fn iter() {
    let x = 3;
    let mut iter = match x {
        1...3 => Left(0..10),
        _ => Right(17..),
    };

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.count(), 9);
}

#[test]
fn read_write() {
    use std::io;

    let use_stdio = false;
    let mockdata = [0xff; 256];

    let mut reader = if use_stdio {
        Left(io::stdin())
    } else {
        Right(&mockdata[..])
    };

    let mut buf = [0u8; 16];
    assert_eq!(reader.read(&mut buf).unwrap(), buf.len());
    assert_eq!(&buf, &mockdata[..buf.len()]);

    let mut mockbuf = [0u8; 256];
    let mut writer = if use_stdio {
        Left(io::stdout())
    } else {
        Right(&mut mockbuf[..])
    };

    let buf = [1u8; 16];
    assert_eq!(writer.write(&buf).unwrap(), buf.len());
}

#[test]
fn error() {
    let invalid_utf8 = b"\xff";
    let res = || -> Result<_, Either<_, _>> {
        try!(::std::str::from_utf8(invalid_utf8).map_err(Left));
        try!("x".parse::<i32>().map_err(Right));
        Ok(())
    }();
    assert!(res.is_err());
    res.unwrap_err().description(); // make sure this can be called
}
