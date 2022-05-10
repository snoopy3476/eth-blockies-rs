extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::String;

/// Trait for canonicalizing Ethereum address, which are used to blockies input
pub trait EthAddr {
    fn addr_as_ref(&self) -> &str;

    /// Convert given Ethereum address string to match the following format:  
    /// * `0x(eth_addr_hex_ascii_lowercase)`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    ///
    /// // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    /// // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// let addr_1 = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC".addr_canonicalize();
    ///
    /// // "e686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// let addr_2 = String::from("e686c14ff9c11038f2b1c9ad617f2346cfb817dc").addr_canonicalize();
    ///
    /// assert_eq!(addr_1, addr_2);
    /// ```
    fn addr_canonicalize(&self) -> String {
        "0x".to_owned()
            + &self
                .addr_as_ref()
                .to_ascii_lowercase()
                .trim_start_matches("0x")
    }
}

impl EthAddr for String {
    fn addr_as_ref(&self) -> &str {
        self.as_str()
    }
}
impl EthAddr for &String {
    fn addr_as_ref(&self) -> &str {
        self.as_str()
    }
}
impl EthAddr for &str {
    fn addr_as_ref(&self) -> &str {
        self
    }
}
