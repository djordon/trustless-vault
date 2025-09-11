use std::vec::Vec;

use sha2::Digest as _;

pub struct BitVec<const MAX_SIZE: u16> {
    data: Vec<u8>,
    len: u16,
}

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
    pub consensus_hash: [u8; 20],
    /// The index block hash of the immediate parent of this block.
    /// This is the hash of the parent block's hash and consensus hash.
    pub parent_block_id: [u8; 32],
    /// The root of a SHA512/256 merkle tree over all this block's
    /// contained transactions
    pub tx_merkle_root: [u8; 32],
    /// The MARF trie root hash after this block has been processed
    pub state_index_root: [u8; 32],
    /// A Unix time timestamp of when this block was mined, according to the miner.
    /// For the signers to consider a block valid, this timestamp must be:
    ///  * Greater than the timestamp of its parent block
    ///  * At most 15 seconds into the future
    pub timestamp: u64,
    /// Recoverable ECDSA signature from the tenure's miner.
    pub miner_signature: [u8; 65],
    /// The set of recoverable ECDSA signatures over
    /// the block header from the signer set active during the tenure.
    /// (ordered by reward set order)
    pub signer_signature: Vec<[u8; 65]>,
    /// A bitvec which conveys whether reward addresses should be punished (by burning their PoX rewards)
    ///  or not in this block.
    ///
    /// The maximum number of entries in the bitvec is 4000.
    pub pox_treatment: BitVec<4000>,
}

impl NakamotoBlockHeader {
    /// Calculate the hash of the block header
    pub fn block_hash(&self) -> [u8; 32] {
        let mut hasher = sha2::Sha512_256::new();
        hasher.update(self.version.to_be_bytes());
        hasher.update(self.chain_length.to_be_bytes());
        hasher.update(self.burn_spent.to_be_bytes());
        hasher.update(self.consensus_hash);
        hasher.update(self.parent_block_id);
        hasher.update(self.tx_merkle_root);
        hasher.update(self.state_index_root);
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(self.miner_signature);
        hasher.update(self.pox_treatment.len.to_be_bytes());
        hasher.update(self.pox_treatment.data.as_slice());
        hasher.finalize().into()
    }

    /// Unique identifier for the block
    pub fn block_id(&self) -> [u8; 32] {
        let mut hasher = sha2::Sha512_256::new();
        hasher.update(self.block_hash());
        hasher.update(self.consensus_hash);
        hasher.finalize().into()
    }
}
