# solana-rust-experiments

This repo is for trying out different tools with basic programs. Curious also about VM bytecode size.

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


## For checking the assembly generated on MacOS M1/M2

```
llvm-objdump -d --arch-name=bpfel target/deploy/program.so
```