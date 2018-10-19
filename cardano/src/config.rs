//! Blockchain network specific config (ProtocolMagic)
//!
//! there are some settings that need to be set in order to guarantee
//! operability with the appropriate network or different option.
//!

use cbor_event::{self, de::RawCbor, se::{Serializer}};
use std::fmt;
use block;
use fee;
use coin;
use redeem;
use address;
use hdwallet;
use std::collections::BTreeMap;

/// this is the protocol magic number
///
/// it is meant to be used on some places in order to guarantee
/// incompatibility between forks, test network and the main-net.
///
/// # Default
///
/// The default value is set to the mainnet
///
/// ```
/// use cardano::config::{ProtocolMagic};
///
/// assert_eq!(ProtocolMagic::default(), ProtocolMagic::new(0x2D964A09));
/// ```
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ProtocolMagic(u32);
impl ProtocolMagic {
    #[deprecated]
    pub fn new(val: u32) -> Self { ProtocolMagic(val) }
}
impl fmt::Display for ProtocolMagic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl ::std::ops::Deref for ProtocolMagic {
    type Target = u32;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl From<u32> for ProtocolMagic {
    fn from(v: u32) -> Self { ProtocolMagic(v) }
}
impl Default for ProtocolMagic {
    fn default() -> Self { ProtocolMagic::from(764824073) }
}
impl cbor_event::se::Serialize for ProtocolMagic {
    fn serialize<W: ::std::io::Write>(&self, serializer: Serializer<W>) -> cbor_event::Result<Serializer<W>> {
        serializer.write_unsigned_integer(self.0 as u64)
    }
}
impl cbor_event::Deserialize for ProtocolMagic {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        let v = raw.unsigned_integer()? as u32;
        Ok(ProtocolMagic::from(v))
    }
}

/// Configuration for the wallet-crypto
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
pub struct Config {
    pub protocol_magic: ProtocolMagic
}
impl Config {
    pub fn new(protocol_magic: ProtocolMagic) -> Self {
        Config {
            protocol_magic: protocol_magic
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Config::new(ProtocolMagic::default())
    }
}

/// Chain configuration parameters. Most of these can be changed
/// through update proposals. (This is cardano-sl's BlockVersionData
/// extended with protocol_magic.)
#[derive(Debug, Clone)]
pub struct ChainParameters {
    pub protocol_magic: ProtocolMagic,
    pub epoch_stability_depth: usize, // a.k.a. 'k'
    pub max_block_size: u64,
    pub max_header_size: u64,
    pub max_tx_size: u64,
    pub max_proposal_size: u64,
    // TODO: why "softfork"? Is there another threshold for hard
    // forks?
    // TODO: use update::SoftforkRule.
    pub softfork_init_thd: Fraction15,
    pub softfork_min_thd: Fraction15,
    pub softfork_thd_decrement: Fraction15,
    pub fee_policy: fee::LinearFee,
    pub update_proposal_thd: Fraction15,
    pub update_vote_thd: Fraction15,
}

// FIXME: use CoinPortion
pub type Fraction15 = u64;

/// A subset of the genesis data. The genesis data is a JSON file
/// whose canonicalized form has the hash 'genesis_prev', which is the
/// parent of the genesis block of epoch 0. (Note that "genesis data"
/// is something completely different from a epoch genesis block. The
/// genesis data is not stored in the chain as a block.)
#[derive(Debug, Clone)]
pub struct GenesisData {
    pub genesis_prev: block::HeaderHash,
    pub avvm_distr: BTreeMap<redeem::PublicKey, coin::Coin>, // AVVM = Ada Voucher Vending Machine
    pub non_avvm_balances: BTreeMap<String, coin::Coin>,
    pub chain_parameters: ChainParameters,
    pub boot_stakeholders: BTreeMap<address::StakeholderId, BootStakeholder>,
}

#[derive(Debug, Clone)]
pub struct BootStakeholder {
    pub weight: BootStakeWeight,
    pub delegate_pk: hdwallet::XPub,
}

pub type BootStakeWeight = u16;
