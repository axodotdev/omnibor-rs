//! Provides a type for representing the hash contained inside a `GitOid`.

use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

/// The hash produced for a `GitOid`
pub struct Hash<'h>(&'h [u8]);

impl<'h> Hash<'h> {
    /// Construct a new `Hash` for the given bytes.
    pub fn new(bytes: &[u8]) -> Hash<'_> {
        Hash(bytes)
    }

    /// Get the hash as a slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0
    }

    /// Get a hexadecimal-encoded representation of the hash.
    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }
}

// Deref to a slice of bytes.
impl<'h> Deref for Hash<'h> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

// Print as the hex encoding.
impl<'h> Display for Hash<'h> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}
