extern crate alloc;

use alloc::{string::String, vec::Vec};

/// Available types of input seed
///
/// Currently supports:
/// * { [Vector](Vec), [array], [slice] } of [bytes](u8)
/// * [String], [str slice](str)
pub trait SeedInput {
    #[doc(hidden)]
    /// Get reference of byte array inside
    fn as_seed_bytes(&self) -> &[u8];

    /// Convert given Ethereum address string or raw bytes data to the following well-formed Ethereum address seed: `0x(hex_letters_lowercase)`
    ///
    /// Apply this function on seeds to generate *standard* Ethereum blockies  
    /// (commonly seen as icons of wallet addresses, on many other Ethereum platforms).
    ///
    /// # Valid input seed
    ///
    /// * Hex string - representing Ethereum address
    ///   * Type: { [Vector](Vec), [array], [slice] } of [bytes](u8), [String], [str slice](str)
    ///   * Length: **40 bytes** (without `0x` prefix), or **42 bytes** (with `0x` prefix)
    /// * Binary bytes data - representing Ethereum address
    ///   * Type: { [Vector](Vec), [array], [slice] } of [bytes](u8)
    ///   * Length: **20 bytes**
    ///
    /// Calling this function on invalid input seeds (other than above) is not an error, but return value should be considered as an undefined, garbage value.  
    /// **Therefore, be sure to VALIDATE YOURSELF whether input seeds are valid before using!**
    ///
    ///
    /// # Example
    ///
    /// * General usage
    /// ```
    /// use eth_blockies::SeedInput;
    ///
    /// // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC" (str slice)
    /// // -> b"0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc" (byte str - 42B)
    /// let addr1 = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///     .to_ethaddr_seed();
    /// assert_eq!(addr1, *b"0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
    ///
    /// // "e686c14ff9c11038f2b1c9ad617f2346cfb817dc" (String)
    /// // -> b"0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc" (byte str - 42B)
    /// let addr2 = String::from("e686c14ff9c11038f2b1c9ad617f2346cfb817dc")
    ///     .to_ethaddr_seed();
    /// assert_eq!(addr1, addr2);
    ///
    /// // 0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc (byte array - 20B)
    /// // -> b"0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc" (byte str - 42B)
    /// let addr3 = [
    ///         0xe6, 0x86, 0xc1, 0x4f, 0xf9, 0xc1, 0x10, 0x38, 0xf2, 0xb1,
    ///         0xc9, 0xad, 0x61, 0x7f, 0x23, 0x46, 0xcf, 0xb8, 0x17, 0xdc,
    ///     ].to_ethaddr_seed();
    /// assert_eq!(addr2, addr3);
    /// ```
    ///
    /// * Calling without `use eth_blockies::SeedInput;` statement
    /// ```
    /// // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    /// // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// let addr = <dyn eth_blockies::SeedInput>::
    ///     to_ethaddr_seed(&"0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC");
    /// assert_eq!(addr, *b"0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
    /// ```
    fn to_ethaddr_seed(&self) -> [u8; 42] {
        self.as_seed_bytes().to_ethaddr_seed()
    }
}

// byte-slice
impl SeedInput for &[u8] {
    fn as_seed_bytes(&self) -> &[u8] {
        *self
    }

    fn to_ethaddr_seed(&self) -> [u8; 42] {
        let seed_data = self.as_seed_bytes();

        match seed_data.len() {
            // str data
            42 => {
                let mut ret: [u8; 42] = (*self).try_into().expect("to_ethaddr_seed()");

                ret.make_ascii_lowercase();

                ret
            }
            // str data without leading '0x'
            40 => {
                let mut ret: [u8; 42] = *b"0x0000000000000000000000000000000000000000";
                let data: [u8; 40] = (*self).try_into().expect("to_ethaddr_seed()");

                ret[2..].copy_from_slice(&data);
                ret.make_ascii_lowercase();

                ret
            }
            // byte data
            20 => {
                let mut ret: [u8; 42] = *b"0x0000000000000000000000000000000000000000";
                let data: [u8; 20] = (*self).try_into().expect("to_ethaddr_seed()");

                const HEX_TABLE: &[u8; 16] = b"0123456789abcdef";
                ret[2..].chunks_exact_mut(2).zip(data.into_iter()).for_each(
                    |(ret_chunk, data_elem)| {
                        ret_chunk.copy_from_slice(&[
                            HEX_TABLE[(data_elem >> 4) as usize],
                            HEX_TABLE[(data_elem & 0x0F) as usize],
                        ]);
                    },
                );

                ret
            }
            // fallback
            _ => [0; 42],
        }
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
