use std::collections::BTreeSet;

use risc0_zkvm::guest::env;
use stacks::blocks::NakamotoBlockHeader;
use stacks::blocks::PublicKey;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let (header, signing_set): (NakamotoBlockHeader, BTreeSet<PublicKey>) = env::read();

    let is_valid = header.verify_signatures(&signing_set);

    let output = (header, signing_set, is_valid);

    println!("was it valid? {:?}", is_valid);
    // write public output to the journal
    env::commit(&output);
}
