# Trustless vaults

This is a demo project that "verifies" that a particular Stacks block header is signed by an expected set of signers.

The public keys hard-coded in the guest program are the public keys associated with the stacks signers' private keys in sBTC's devenv.

The block header encoded in the host program was generated using the stacks node in sBTC's devenv. Specifically, after running `make devenv-up` in the sBTC repo and waiting for Nakamoto, I ran the following rust code as a test in the test module of `signer/src/stacks/api.rs`, which prints the block header:
```rust
#[tokio::test]
async fn get_blocks_test2() {
    // Here we test that out code will handle the response from a
    // stacks node in the expected way.
    const TENURE_END_BLOCK_ID: &str =
        "B0E21FEC1677FBF46392B2C83B9C383A7CBA6EF083C651F05680F984C33FD62F";

    let client = StacksClient::new(url::Url::parse("http://localhost:20443").unwrap()).unwrap();

    let block_id = StacksBlockId::from_hex(TENURE_END_BLOCK_ID).unwrap();
    // The moment of truth, do the requests succeed?
    let block = client.get_block(block_id).await.unwrap();
    
    println!("{}", block.is_shadow_block());
    println!("{}", serde_json::to_string(&block.header).unwrap());
}
```
The public keys in the guest program were generated as follows
```rust
    let signer_1 =
        hex::decode("41634762d89dfa09133a4a8e9c1378d0161d29cd0a9433b51f1e3d32947a73dc").unwrap();
    let signer_2 =
        hex::decode("9bfecf16c9c12792589dd2b843f850d5b89b81a04f8ab91c083bdf6709fbefee").unwrap();
    let signer_3 =
        hex::decode("3ec0ca5770a356d6cd1a9bfcbf6cd151eb1bd85c388cc00648ec4ef5853fdb74").unwrap();

    let verifying_key_1 = k256::SecretKey::from_slice(&signer_1).unwrap();
    let verifying_key_2 = k256::SecretKey::from_slice(&signer_2).unwrap();
    let verifying_key_3 = k256::SecretKey::from_slice(&signer_3).unwrap();

    let signing_set: BTreeSet<PublicKey> = [verifying_key_1, verifying_key_2, verifying_key_3]
        .iter()
        .map(|sk| sk.public_key().into())
        .collect();

    println!("signing set: {:?}", signing_set.iter().next().unwrap());
    println!("signing set: {:?}", signing_set.iter().nth(1).unwrap());
    println!("signing set: {:?}", signing_set.iter().nth(2).unwrap());
```

To run the code here without generating a proof, do the following:
```bash
RISC0_DEV_MODE=1 RUST_LOG="info" RISC0_INFO=1 cargo run --release
```
To generate a proof set `RISC0_DEV_MODE=0` and run the host binary with dev mode set to zero:
```bash
RISC0_DEV_MODE=0 RUST_LOG="info" RISC0_INFO=1 ./target/release/host
```
The above command should take a few minutes.


---
---

# RISC Zero Rust Starter Template

Welcome to the RISC Zero Rust Starter Template! This template is intended to
give you a starting point for building a project using the RISC Zero zkVM.
Throughout the template (including in this README), you'll find comments
labelled `TODO` in places where you'll need to make changes. To better
understand the concepts behind this template, check out the [zkVM
Overview][zkvm-overview].

## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following
command:

```bash
cargo run
```

This is an empty template, and so there is no expected output (until you modify
the code).

### Executing the Project Locally in Development Mode

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest activating it during your early development phase. Furthermore, you might want to get insights into the execution statistics of your project, and this can be achieved by specifying the environment variable `RUST_LOG="[executor]=info"` before running your project.

Put together, the command to run your project in development mode while getting execution statistics is:

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
```

### Running Proofs Remotely on Bonsai

_Note: The Bonsai proving service is still in early Alpha; an API key is
required for access. [Click here to request access][bonsai access]._

If you have access to the URL and API key to Bonsai you can run your proofs
remotely. To prove in Bonsai mode, invoke `cargo run` with two additional
environment variables:

```bash
BONSAI_API_KEY="YOUR_API_KEY" BONSAI_API_URL="BONSAI_URL" cargo run
```

## How to Create a Project Based on This Template

Search this template for the string `TODO`, and make the necessary changes to
implement the required feature described by the `TODO` comment. Some of these
changes will be complex, and so we have a number of instructional resources to
assist you in learning how to write your own code for the RISC Zero zkVM:

- The [RISC Zero Developer Docs][dev-docs] is a great place to get started.
- Example projects are available in the [examples folder][examples] of
  [`risc0`][risc0-repo] repository.
- Reference documentation is available at [https://docs.rs][docs.rs], including
  [`risc0-zkvm`][risc0-zkvm], [`cargo-risczero`][cargo-risczero],
  [`risc0-build`][risc0-build], and [others][crates].

## Directory Structure

It is possible to organize the files for these components in various ways.
However, in this starter template we use a standard directory structure for zkVM
applications, which we think is a good starting point for your applications.

```text
project_name
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs                    <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── method_name.rs         <-- [Guest code goes here]
    └── src
        └── lib.rs
```

## Video Tutorial

For a walk-through of how to build with this template, check out this [excerpt
from our workshop at ZK HACK III][zkhack-iii].

## Questions, Feedback, and Collaborations

We'd love to hear from you on [Discord][discord] or [Twitter][twitter].

[bonsai access]: https://bonsai.xyz/apply
[cargo-risczero]: https://docs.rs/cargo-risczero
[crates]: https://github.com/risc0/risc0/blob/main/README.md#rust-binaries
[dev-docs]: https://dev.risczero.com
[dev-mode]: https://dev.risczero.com/api/generating-proofs/dev-mode
[discord]: https://discord.gg/risczero
[docs.rs]: https://docs.rs/releases/search?query=risc0
[examples]: https://github.com/risc0/risc0/tree/main/examples
[risc0-build]: https://docs.rs/risc0-build
[risc0-repo]: https://www.github.com/risc0/risc0
[risc0-zkvm]: https://docs.rs/risc0-zkvm
[rust-toolchain]: rust-toolchain.toml
[rustup]: https://rustup.rs
[twitter]: https://twitter.com/risczero
[zkhack-iii]: https://www.youtube.com/watch?v=Yg_BGqj_6lg&list=PLcPzhUaCxlCgig7ofeARMPwQ8vbuD6hC5&index=5
[zkvm-overview]: https://dev.risczero.com/zkvm
