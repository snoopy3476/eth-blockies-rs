extern crate alloc;

use alloc::{string::String, vec::Vec};

/// Trait for blockies input seed
pub trait SeedInput {
    /// Get reference of byte array inside
    fn as_seed_bytes(&self) -> &[u8];
}

// byte-slice
impl SeedInput for &[u8] {
    fn as_seed_bytes(&self) -> &[u8] {
        *self
    }
}

// byte-array
impl<const N: usize> SeedInput for [u8; N] {
    fn as_seed_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}
impl<const N: usize> SeedInput for &[u8; N] {
    fn as_seed_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

// byte-vec
impl SeedInput for Vec<u8> {
    fn as_seed_bytes(&self) -> &[u8] {
        self.as_slice()
    }
}
impl SeedInput for &Vec<u8> {
    fn as_seed_bytes(&self) -> &[u8] {
        (*self).as_slice()
    }
}

// string-slice
impl SeedInput for &str {
    fn as_seed_bytes(&self) -> &[u8] {
        (*self).as_bytes()
    }
}

// string
impl SeedInput for String {
    fn as_seed_bytes(&self) -> &[u8] {
        (*self).as_bytes()
    }
}
impl SeedInput for &String {
    fn as_seed_bytes(&self) -> &[u8] {
        (**self).as_seed_bytes()
    }
}

/// Trait for blockies input string seed
pub trait SeedString {
    /// Get reference of string slice inside
    fn as_seed_str(&self) -> &str;

    /// Convert given Ethereum address string to match the following format:  
    /// * `0x(hex_letters_lowercase)`
    ///
    /// Using this function on non-Ethereum-address string is not an error,
    /// but it is highly likely that the returned result is meaningless.
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    ///
    /// // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    /// // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// let addr_1 = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC".canonicalize_ethaddr();
    ///
    /// assert_eq!(addr_1, "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
    ///
    /// // "e686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// let addr_2 = String::from("e686c14ff9c11038f2b1c9ad617f2346cfb817dc").canonicalize_ethaddr();
    ///
    /// assert_eq!(addr_1, addr_2);
    /// ```
    fn canonicalize_ethaddr(&self) -> String {
        let lowercase = self.as_seed_str().to_ascii_lowercase();
        match lowercase.starts_with("0x") {
            true => lowercase,
            false => ["0x", lowercase.as_ref()].concat(),
        }
    }

    #[doc(hidden)]
    #[deprecated(note = "Function name is changed to: 'canonicalize_ethaddr()'")]
    fn addr_canonicalize(&self) -> String {
        self.canonicalize_ethaddr()
    }
}

// string-slice
impl SeedString for &str {
    fn as_seed_str(&self) -> &str {
        *self
    }
}

// string
impl SeedString for String {
    fn as_seed_str(&self) -> &str {
        (*self).as_str()
    }
}
impl SeedString for &String {
    fn as_seed_str(&self) -> &str {
        (**self).as_str()
    }
}
