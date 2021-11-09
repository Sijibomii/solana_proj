### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```

### Could not test on windows
Link to tutorial: https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/#processor-part-2-pdas-part-2-cpis-part-1
