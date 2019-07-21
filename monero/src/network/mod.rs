use crate::address::Format;
use wagu_model::{AddressError, Network};

pub mod mainnet;
pub use self::mainnet::*;

pub mod stagenet;
pub use self::stagenet::*;

pub mod testnet;
pub use self::testnet::*;

/// The interface for a Monero network.
pub trait MoneroNetwork: Network {
    /// Returns the address prefix of the given network.
    fn to_address_prefix(format: &Format) -> u8;

    /// Returns the network of the given address prefix.
    fn from_address_prefix(prefix: u8) -> Result<Self, AddressError>;
}
