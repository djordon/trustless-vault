use std::collections::BTreeSet;

use k256::ecdsa::VerifyingKey;
use risc0_zkvm::guest::env;
use stacks::blocks::NakamotoBlockHeader;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let (header, signing_set): (NakamotoBlockHeader, BTreeSet<VerifyingKey>) = env::read();

    let is_valid = header.verify_signatures(&signing_set);

    let output = (header, signing_set, is_valid);

    // write public output to the journal
    env::commit(&output);
}
