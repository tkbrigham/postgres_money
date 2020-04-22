// pub(crate) mod error;
// pub(crate) use self::error::Error;

use crate::Money;
use std::io::Error;

impl Money {
    pub const MAX_LEN_NEG: usize = "-92233720368547758.08".len();
    pub const MAX_LEN_POS: usize = "92233720368547758.07".len();
    pub fn parse_str(input: &str) -> Result<Money, Error> {
        Ok(Money(1337))
    }
}
