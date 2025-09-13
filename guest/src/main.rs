use risc0_zkvm::guest::env;
use stacks::blocks::NakamotoBlockHeader;
use stacks::blocks::PublicKey;
use stacks::blocks::FixedArray;

const SIGNER_1_PUBLIC_KEY: [u8; 33] = [
    2,   0, 115,  17,  67,   1,  35, 212, 202, 217, 127,
   79, 126, 134, 224,  35, 178, 129,  67,  19,  10,  24,
    9, 158, 207,   9,  77,  54, 254, 240, 246,  19,  92,
];
const SIGNER_2_PUBLIC_KEY: [u8; 33] = [
    3,  26,  77, 159,  73,   3, 218, 151,  73, 137,  69,
  164, 224,  26,  80,  35, 161, 213,  59, 201, 106, 214,
  112, 191, 224,  58, 223, 138,   6, 197,  46,  99, 128,
];

const SIGNER_3_PUBLIC_KEY: [u8; 33] = [
    3,  82,  73,  19, 114, 134, 192, 119, 204, 238, 101,
  236, 196,  62, 114,  75, 155, 158,  90,  88, 142,  61,
  127,  81, 227, 182,  47, 150,  36, 194, 164, 158,  70,
];

const INITIAL_SIGNING_SET: [PublicKey; 3] = [
    PublicKey(FixedArray(SIGNER_1_PUBLIC_KEY)),
    PublicKey(FixedArray(SIGNER_2_PUBLIC_KEY)),
    PublicKey(FixedArray(SIGNER_3_PUBLIC_KEY)),
];

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let header: NakamotoBlockHeader = env::read();

    let is_valid = header.verify_signatures(&INITIAL_SIGNING_SET);

    let output = (header,is_valid);

    println!("was it valid? {:?}", is_valid);
    // write public output to the journal
    env::commit(&output);
}
