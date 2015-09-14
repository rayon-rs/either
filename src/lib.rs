use std::io::{self, Write, Read};

pub use Either::{Left, Right};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

macro_rules! either_mut {
    ($value:expr, $inner:ident => $result:expr) => (
        match $value {
            Either::Left(ref mut $inner) => $result,
            Either::Right(ref mut $inner) => $result,
        }
    )
}

impl<L, R> Iterator for Either<L, R>
    where L: Iterator, R: Iterator<Item=L::Item>
{
    type Item = L::Item;

    fn next(&mut self) -> Option<L::Item> {
        either_mut!(*self, inner => inner.next())
    }
}

impl<L, R> Read for Either<L, R>
    where L: Read, R: Read
{
    fn read(&mut self, data: &mut [u8]) -> io::Result<usize> {
        either_mut!(*self, inner => inner.read(data))
    }
}

impl<L, R> Write for Either<L, R>
    where L: Write, R: Write
{
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        either_mut!(*self, inner => inner.write(data))
    }

    fn flush(&mut self) -> io::Result<()> {
        either_mut!(*self, inner => inner.flush())
    }
}

#[test]
fn it_works() {
}
