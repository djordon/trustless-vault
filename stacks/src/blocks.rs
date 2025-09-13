use std::vec::Vec;

use k256::CompressedPoint;
use k256::ecdsa::RecoveryId;
use k256::ecdsa::Signature;
use k256::ecdsa::VerifyingKey;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

pub struct BitVec {
    pub data: Vec<u8>,
    pub len: u16,
}

impl serde::Serialize for BitVec {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut len = self.len.to_be_bytes().to_vec();
        len.extend_from_slice(&self.data);
        let inst = hex::encode(len);
        s.serialize_str(inst.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for BitVec {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<BitVec, D::Error> {
        let inst_str = String::deserialize(d)?;
        let bytes = hex::decode(&inst_str).map_err(serde::de::Error::custom)?;

        let len = u16::from_be_bytes(bytes[..2].try_into().unwrap());
        let data = bytes[2..].to_vec();
        Ok(BitVec { len, data })
    }
}

#[derive(Deserialize, Serialize)]
pub struct NakamotoBlockHeader {
    pub version: u8,
    /// The total number of StacksBlock and NakamotoBlocks preceding
    /// this block in this block's history.
    pub chain_length: u64,
    /// Total amount of BTC spent producing the sortition that
    /// selected this block's miner.
    pub burn_spent: u64,
    /// The consensus hash of the burnchain block that selected this tenure.  The consensus hash
    /// uniquely identifies this tenure, including across all Bitcoin forks.
    pub consensus_hash: FixedArray<20>,
    /// The index block hash of the immediate parent of this block.
    /// This is the hash of the parent block's hash and consensus hash.
    pub parent_block_id: FixedArray<32>,
    /// The root of a SHA512/256 merkle tree over all this block's
    /// contained transactions
    pub tx_merkle_root: FixedArray<32>,
    /// The MARF trie root hash after this block has been processed
    pub state_index_root: FixedArray<32>,
    /// A Unix time timestamp of when this block was mined, according to the miner.
    /// For the signers to consider a block valid, this timestamp must be:
    ///  * Greater than the timestamp of its parent block
    ///  * At most 15 seconds into the future
    pub timestamp: u64,
    /// Recoverable ECDSA signature from the tenure's miner.
    pub miner_signature: FixedArray<65>,
    /// The set of recoverable ECDSA signatures over
    /// the block header from the signer set active during the tenure.
    /// (ordered by reward set order)
    pub signer_signature: Vec<RecoverableSignature>,
    /// A bitvec which conveys whether reward addresses should be punished (by burning their PoX rewards)
    ///  or not in this block.
    ///
    /// The maximum number of entries in the bitvec is 4000.
    pub pox_treatment: BitVec,
}

impl NakamotoBlockHeader {
    /// Calculate the hash of the block header
    pub fn block_hash(&self) -> [u8; 32] {
        let mut hasher = sha2::Sha512_256::new();
        hasher.update(self.version.to_be_bytes());
        hasher.update(self.chain_length.to_be_bytes());
        hasher.update(self.burn_spent.to_be_bytes());
        hasher.update(self.consensus_hash.0);
        hasher.update(self.parent_block_id.0);
        hasher.update(self.tx_merkle_root.0);
        hasher.update(self.state_index_root.0);
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(self.miner_signature.0);
        hasher.update(self.pox_treatment.len.to_be_bytes());
        hasher.update(self.pox_treatment.data.as_slice());
        hasher.finalize().into()
    }

    /// Unique identifier for the block
    pub fn block_id(&self) -> [u8; 32] {
        let mut hasher = sha2::Sha512_256::new();
        hasher.update(self.block_hash());
        hasher.update(self.consensus_hash.0);
        hasher.finalize().into()
    }

    /// Get the signing weight of a shadow block
    pub fn get_shadow_signer_weight(&self, reward_set: &RewardSet) -> u32 {
        reward_set
            .signers
            .iter()
            .fold(0u32, |acc, signer| acc.saturating_add(signer.weight))
    }

    pub fn is_shadow_block(&self) -> bool {
        self.version & 0x80 != 0
    }

    pub fn verify_signatures(&self, signing_set: &[PublicKey; 3]) -> bool {
        let hash = self.block_hash();

        if self.signer_signature.is_empty() {
            return false;
        }

        self.signer_signature
            .iter()
            .map(|sig| sig.verifying_key(&hash))
            .map(PublicKey::from)
            .all(|key| signing_set.contains(&key))
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct FixedArray<const N: usize>(pub [u8; N]);

impl<const N: usize> serde::Serialize for FixedArray<N> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let inst = hex::encode(self.0);
        s.serialize_str(inst.as_str())
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for FixedArray<N> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<FixedArray<N>, D::Error> {
        let inst_str = String::deserialize(d)?;
        let bytes: [u8; N] = hex::decode(&inst_str)
            .map_err(serde::de::Error::custom)?
            .try_into()
            .map_err(|bytes| serde::de::Error::custom(hex::encode(bytes)))?;

        Ok(FixedArray(bytes))
    }
}

#[derive(Deserialize, Serialize)]
pub struct RecoverableSignature(pub FixedArray<65>);

impl RecoverableSignature {
    pub fn signature(&self) -> (Signature, RecoveryId) {
        let recovery_id = RecoveryId::from_byte(self.0.0[0]).unwrap();
        (Signature::from_slice(&self.0.0[1..]).unwrap(), recovery_id)
    }

    pub fn verifying_key(&self, msg: &[u8; 32]) -> VerifyingKey {
        let (signature, recovery_id) = self.signature();
        VerifyingKey::recover_from_prehash(msg, &signature, recovery_id).unwrap()
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct PublicKey(pub FixedArray<33>);

impl From<k256::PublicKey> for PublicKey {
    fn from(public_key: k256::PublicKey) -> Self {
        PublicKey(FixedArray(CompressedPoint::from(public_key).into()))
    }
}

impl From<VerifyingKey> for PublicKey {
    fn from(verifying_key: VerifyingKey) -> Self {
        PublicKey::from(k256::PublicKey::from(verifying_key))
    }
}

pub struct NakamotoSignerEntry {
    pub signing_key: [u8; 33],
    pub stacked_amt: u128,
    pub weight: u32,
}

pub struct RewardSet {
    pub signers: Vec<NakamotoSignerEntry>,
    pub pox_ustx_threshold: Option<u128>,
}
