use crate::Money;

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{BufMut, BytesMut};
use postgres_types::{FromSql, IsNull, ToSql, Type};
use std::error::Error;

impl<'a> FromSql<'a> for Money {
    fn from_sql(_: &Type, mut buf: &[u8]) -> Result<Money, Box<dyn Error + Sync + Send>> {
        let v = buf.read_i64::<BigEndian>()?;
        if !buf.is_empty() {
            return Err("invalid buffer size".into());
        }
        Ok(Money::from(v))
    }

    postgres_types::accepts!(MONEY);
}

impl ToSql for Money {
    fn to_sql(&self, _: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        w.put_i64(self.inner());
        Ok(IsNull::No)
    }

    postgres_types::accepts!(MONEY);
    postgres_types::to_sql_checked!();
}
