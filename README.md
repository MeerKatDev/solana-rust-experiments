# solana-rust-experiments

This repo is for trying out different tools with basic programs. Curious also about VM bytecode size.

# araiza-messenger

Messenger test app created by [this author](https://github.com/JacobAraiza). It has anchor and tests with solana-program-test. Works with `cargo test-sbf`.

# araiza-nft

NFT minting test app created by [this author](https://github.com/JacobAraiza). It has anchor and tests with solana-sdk, i.e. it's using Rust Client methods basically.

## counter-mollusk 
Simple counter solana-program app with a [mollusk](https://github.com/anza-xyz/mollusk) test.

Run with 
```
cargo build-sbf
cargo test
```

## pinocchio-counter-mollusk 
Simple counter pinocchio app with a [mollusk](https://github.com/anza-xyz/mollusk) test.

Run with 
```
cargo build-sbf
cargo test
```

## counter-litesvm
Simple counter solana-program app with [litesvm](https://github.com/LiteSVM) test.
Run with 
```
cargo build-sbf
cargo test
```

## pinocchio-favorites-spt
Program with pinocchio and pinocchio-pubkey running solana program test, minimal test setup. Run with `cargo test-sbf`.

## native-vault

This example was taken from [Pinocchio guide](https://github.com/vict0rcarvalh0/pinocchio-guide) and corrected. It contains create account for wallet and pdas, and two examples of transfer (one for wallets and one for pdas) with its respective tests.


## For checking the assembly generated on MacOS M1/M2

```
llvm-objdump -d --arch-name=bpfel target/deploy/program.so
```