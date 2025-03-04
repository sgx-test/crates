//! Child numbers

use crate::{Error, Result};
use core::str::FromStr;

/// Hardened child keys use indices 2^31 through 2^32-1.
const HARDENED_FLAG: u32 = 1 << 31;

/// Index of a particular child key for a given (extended) secret key.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ChildNumber(u32);

impl ChildNumber {
    /// Is this child number within the hardened range?
    pub fn is_hardened(&self) -> bool {
        self.0 & HARDENED_FLAG != 0
    }

    /// Serialize this child number as bytes.
    pub fn to_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

impl FromStr for ChildNumber {
    type Err = Error;

    fn from_str(child: &str) -> Result<ChildNumber> {
        let (child, mask) = match child.strip_suffix('\'') {
            Some(c) => (c, HARDENED_FLAG),
            None => (child, 0),
        };

        let index = child.parse::<u32>().map_err(|_| Error)?;

        if index & HARDENED_FLAG == 0 {
            Ok(ChildNumber(index | mask))
        } else {
            Err(Error)
        }
    }
}
