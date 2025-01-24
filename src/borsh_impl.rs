use crate::Either;
use borsh::{BorshDeserialize, BorshSerialize};

impl<L, R> BorshSerialize for Either<L, R>
where
    L: BorshSerialize,
    R: BorshSerialize,
{
    fn serialize<Writer: borsh::io::Write>(
        &self,
        writer: &mut Writer,
    ) -> Result<(), borsh::io::Error> {
        match self {
            Either::Left(l) => {
                false.serialize(writer)?;
                l.serialize(writer)?;
            }
            Either::Right(r) => {
                true.serialize(writer)?;
                r.serialize(writer)?;
            }
        }
        Ok(())
    }
}
impl<L, R> BorshDeserialize for Either<L, R>
where
    L: BorshDeserialize,
    R: BorshDeserialize,
{
    fn deserialize_reader<Reader: borsh::io::Read>(
        reader: &mut Reader,
    ) -> Result<Self, borsh::io::Error> {
        let tag = bool::deserialize_reader(reader)?;
        if tag {
            Ok(Self::Right(R::deserialize_reader(reader)?))
        } else {
            Ok(Self::Left(L::deserialize_reader(reader)?))
        }
    }
}

#[cfg(test)]
mod tests {
    type U8OrU128 = crate::Either<u8, u128>;
    #[test]
    fn encdec() {
        let l = U8OrU128::Left(42);
        let r = U8OrU128::Right(1234567);
        let l_bytes = borsh::to_vec(&l).unwrap();
        let r_bytes = borsh::to_vec(&r).unwrap();
        let l2 = borsh::from_slice(&l_bytes).unwrap();
        let r2 = borsh::from_slice(&r_bytes).unwrap();
        assert_eq!(l, l2);
        assert_eq!(r, r2);
    }
}
