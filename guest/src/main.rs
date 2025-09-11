use risc0_zkvm::guest::env;
use stacks::blocks::NakamotoBlockHeader;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: NakamotoBlockHeader = env::read();

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
