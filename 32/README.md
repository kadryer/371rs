# KUDOS
## Outline
This "32" repository provides the baseline to code a "Bare Metal" program in Rust. The starting file, `main.rs`, is effectively empty. This document will walk you through how to get this crate to work once you've installed it. I assume that you use a WSL Ubuntu environment, though it may not be required.

## Dependencies
This crate depends on the RISC-V Bare-Metal environment `riscv64imac-unknown-none-elf`, which can be installed with: 
```{.sh}
rustup target add riscv64imac-unknown-none-elf
```
Then, you will need to install QEMU package for RISC-V:
```{.sh}
sudo apt install qemu-system-misc
```
If this does not work, try `sudo apt update` and `sudo apt upgrade`, then run it again.

## Running Files
Once those are passed, you may run files with the following command:
```{.sh}
qemu-system-riscv64 -machine sifive_u -bios none -kernel YOUR_FILE
```

## Note
I'm not sure how to verify this installation sequence for myself. I believe every other step I took to make this is included within the files as saved arguments (such as the step `cargo build --target riscv64imac-unknown-none-elf` after adding it in rustup, or adding the -C arguments in `rustc -C link-args=-Tmemory.x -C link-args=-Tlink.x -C panic=abort src/main.rs`). Please let me know if this is an oversight.
