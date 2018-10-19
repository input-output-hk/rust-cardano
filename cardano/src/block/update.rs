use cbor_event::{self, de::RawCbor};
use hash;
use hdwallet;
use std::collections::{BTreeMap};
use super::types;

#[derive(Debug, Clone)]
pub struct UpdatePayload {
    pub proposal: Option<UpdateProposal>,
    pub votes: Vec<UpdateVote>,
}

impl cbor_event::se::Serialize for UpdatePayload {
    fn serialize<W: ::std::io::Write>(&self, serializer: cbor_event::se::Serializer<W>) -> cbor_event::Result<cbor_event::se::Serializer<W>> {
        let serializer = serializer.write_array(cbor_event::Len::Len(2))?
            .serialize(&self.proposal)?;
        cbor_event::se::serialize_indefinite_array(self.votes.iter(), serializer)
    }
}

impl cbor_event::de::Deserialize for UpdatePayload {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(2, "UpdatePayload")?;
        Ok(Self {
            proposal: raw.deserialize()?,
            votes: raw.deserialize()?
        })
    }
}

#[derive(Debug, Clone)]
pub struct UpdateProposal {
    pub block_version: types::BlockVersion,
    pub block_version_mod: BlockVersionModifier,
    pub software_version: types::SoftwareVersion,
    pub data: BTreeMap<SystemTag, UpdateData>,
    pub attributes: UpAttributes,
    pub from: hdwallet::XPub,
    pub signature: hdwallet::Signature<()> // UpdateProposalToSign
}

pub type UpAttributes = types::Attributes;
pub type SystemTag = String;

impl cbor_event::se::Serialize for UpdateProposal {
    fn serialize<W: ::std::io::Write>(&self, serializer: cbor_event::se::Serializer<W>) -> cbor_event::Result<cbor_event::se::Serializer<W>> {
        let serializer = serializer.write_array(cbor_event::Len::Len(7))?
            .serialize(&self.block_version)?
            .serialize(&self.block_version_mod)?
            .serialize(&self.software_version)?;
        cbor_event::se::serialize_fixed_map(self.data.iter(), serializer)?
            .serialize(&self.attributes)?
            .serialize(&self.from)?
            .serialize(&self.signature)
    }
}

impl cbor_event::de::Deserialize for UpdateProposal {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(7, "UpdateProposal")?;
        Ok(Self {
            block_version: raw.deserialize()?,
            block_version_mod: raw.deserialize()?,
            software_version: raw.deserialize()?,
            data: raw.deserialize()?,
            attributes: raw.deserialize()?,
            from: raw.deserialize()?,
            signature: raw.deserialize()?
        })
    }
}

#[derive(Debug, Clone)]
pub struct BlockVersionModifier {
    pub script_version: Option<ScriptVersion>,
    pub slot_duration: Option<Millisecond>,
    pub max_block_size: Option<u64>,
    pub max_header_size: Option<u64>,
    pub max_tx_size: Option<u64>,
    pub max_proposal_size: Option<u64>,
    pub mpc_thd: Option<types::CoinPortion>,
    pub heavy_del_thd: Option<types::CoinPortion>,
    pub update_vote_thd: Option<types::CoinPortion>,
    pub update_proposal_thd: Option<types::CoinPortion>,
    pub update_implicit: Option<FlatSlotId>,
    pub softfork_rule: Option<SoftforkRule>,
    pub tx_fee_policy: Option<TxFeePolicy>,
    pub unlock_stake_epoch: Option<types::EpochId>,
}

impl cbor_event::se::Serialize for BlockVersionModifier {
    fn serialize<W: ::std::io::Write>(&self, serializer: cbor_event::se::Serializer<W>) -> cbor_event::Result<cbor_event::se::Serializer<W>> {
        assert!(self.tx_fee_policy.is_none()); // not tested yet
        serializer.write_array(cbor_event::Len::Len(14))?
            .serialize(&self.script_version)?
            .serialize(&self.slot_duration)?
            .serialize(&self.max_block_size)?
            .serialize(&self.max_header_size)?
            .serialize(&self.max_tx_size)?
            .serialize(&self.max_proposal_size)?
            .serialize(&self.mpc_thd)?
            .serialize(&self.heavy_del_thd)?
            .serialize(&self.update_vote_thd)?
            .serialize(&self.update_proposal_thd)?
            .serialize(&self.update_implicit)?
            .serialize(&self.softfork_rule)?
            .serialize(&self.tx_fee_policy)?
            .serialize(&self.unlock_stake_epoch)
    }
}

impl cbor_event::de::Deserialize for BlockVersionModifier {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(14, "BlockVersionModifier")?;
        Ok(Self {
            script_version: raw.deserialize()?,
            slot_duration: raw.deserialize()?,
            max_block_size: raw.deserialize()?,
            max_header_size: raw.deserialize()?,
            max_tx_size: raw.deserialize()?,
            max_proposal_size: raw.deserialize()?,
            mpc_thd: raw.deserialize()?,
            heavy_del_thd: raw.deserialize()?,
            update_vote_thd: raw.deserialize()?,
            update_proposal_thd: raw.deserialize()?,
            update_implicit: raw.deserialize()?,
            softfork_rule: raw.deserialize()?,
            tx_fee_policy: raw.deserialize()?,
            unlock_stake_epoch: raw.deserialize()?
        })
    }
}

pub type ScriptVersion = u16;
pub type Millisecond = u64;
pub type FlatSlotId = u64;
pub type TxFeePolicy = cbor_event::Value; // TODO

#[derive(Debug, Clone)]
pub struct UpdateData {
    pub app_diff_hash: hash::Blake2b256,
    pub pkg_hash: hash::Blake2b256,
    pub updater_hash: hash::Blake2b256,
    pub metadata_hash: hash::Blake2b256,
}

impl cbor_event::se::Serialize for UpdateData {
    fn serialize<W: ::std::io::Write>(&self, serializer: cbor_event::se::Serializer<W>) -> cbor_event::Result<cbor_event::se::Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?
            .serialize(&self.app_diff_hash)?
            .serialize(&self.pkg_hash)?
            .serialize(&self.updater_hash)?
            .serialize(&self.metadata_hash)
    }
}

impl cbor_event::de::Deserialize for UpdateData {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(4, "UpdateData")?;
        Ok(Self {
            app_diff_hash: raw.deserialize()?,
            pkg_hash: raw.deserialize()?,
            updater_hash: raw.deserialize()?,
            metadata_hash: raw.deserialize()?
        })
    }
}

#[derive(Debug, Clone)]
pub struct SoftforkRule {
    pub init_thd: types::CoinPortion,
    pub min_thd: types::CoinPortion,
    pub thd_decrement: types::CoinPortion,
}

impl cbor_event::se::Serialize for SoftforkRule {
    fn serialize<W: ::std::io::Write>(&self, serializer: cbor_event::se::Serializer<W>) -> cbor_event::Result<cbor_event::se::Serializer<W>> {
        serializer.serialize(&(&self.init_thd, &self.min_thd, &self.thd_decrement))
    }
}

impl cbor_event::de::Deserialize for SoftforkRule {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(3, "SoftforkRule")?;
        Ok(Self {
            init_thd: raw.deserialize()?,
            min_thd: raw.deserialize()?,
            thd_decrement: raw.deserialize()?
        })
    }
}

#[derive(Debug, Clone)]
pub struct UpdateVote {
    pub key: hdwallet::XPub,
    pub proposal_id: UpId,
    pub decision: bool,
    pub signature: hdwallet::Signature<(UpId, bool)>,
}

pub type UpId = hash::Blake2b256; // UpdateProposal

impl cbor_event::se::Serialize for UpdateVote {
    fn serialize<W: ::std::io::Write>(&self, serializer: cbor_event::se::Serializer<W>) -> cbor_event::Result<cbor_event::se::Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?
            .serialize(&self.key)?
            .serialize(&self.proposal_id)?
            .serialize(&self.decision)?
            .serialize(&self.signature)
    }
}

impl cbor_event::de::Deserialize for UpdateVote {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> cbor_event::Result<Self> {
        raw.tuple(4, "UpdateVote")?;
        Ok(Self {
            key: raw.deserialize()?,
            proposal_id: raw.deserialize()?,
            decision: raw.deserialize()?,
            signature: raw.deserialize()?
        })
    }
}

#[derive(Debug, Clone)]
pub struct UpdateProposalToSign<'a> {
    pub block_version: &'a types::BlockVersion,
    pub block_version_mod: &'a BlockVersionModifier,
    pub software_version: &'a types::SoftwareVersion,
    pub data: &'a BTreeMap<SystemTag, UpdateData>,
    pub attributes: &'a UpAttributes,
}

impl<'a> cbor_event::se::Serialize for UpdateProposalToSign<'a> {
    fn serialize<W: ::std::io::Write>(&self, serializer: cbor_event::se::Serializer<W>) -> cbor_event::Result<cbor_event::se::Serializer<W>> {
        let serializer = serializer.write_array(cbor_event::Len::Len(5))?
            .serialize(&self.block_version)?
            .serialize(&self.block_version_mod)?
            .serialize(&self.software_version)?;
        cbor_event::se::serialize_fixed_map(self.data.iter(), serializer)?
            .serialize(&self.attributes)
    }
}

