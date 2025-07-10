```
cargo objdump -- -h target/thumbv7em-none-eabihf/debug/microcontroller-rust

cargo readobj -- -S target/thumbv7em-none-eabihf/debug/microcontroller-rust

// Print all details of the ELF file
cargo readobj -- -all target/thumbv7em-none-eabihf/debug/microcontroller-rust

// Print the contents of each section in an ELF file
cargo readobj -- -x .data target/thumbv7em-none-eabihf/debug/microcontroller-rust

// Display the symbol table
cargo readobj -- -s target/thumbv7em-none-eabihf/debug/microcontroller-rust
```
