# Setup Environment

## Target Setup
Based on the different architecture of MCU, need to install below target.

Cortex-M0, M0+, and M1 (ARMv6-M architecture):
```
rustup target add thumbv6m-none-eabi
```

Cortex-M3 (ARMv7-M architecture):
```
rustup target add thumbv7m-none-eabi
```

Cortex-M4 and M7 without hardware floating point (ARMv7E-M architecture):
```
rustup target add thumbv7em-none-eabi
```

Cortex-M4F and M7F with hardware floating point (ARMv7E-M architecture):
```
rustup target add thumbv7em-none-eabihf
```

Cortex-M23 (ARMv8-M architecture):
```
rustup target add thumbv8m.base-none-eabi
```

Cortex-M33 and M35P (ARMv8-M architecture):
```
rustup target add thumbv8m.main-none-eabi
```

Cortex-M33F and M35PF with hardware floating point (ARMv8-M architecture):
```
rustup target add thumbv8m.main-none-eabihf
```

## Cargo-binutils

```
cargo install cargo-binutils
rustup component add llvm-tools
cargo binstall probe-rs-tools
```

### Erase Chip
```
❯ probe-rs list
The following debug probes were found:
[0]: DAPLink CMSIS_DAP (VID: 0d28, PID: 0204, Serial: 0604000048824e45005e7007dd89002f8761000097969900, CmsisDap)
❯ probe-rs erase --chip STM32F469NIHx --probe 0d28:0204

```

# Add crate dependency

Add crates
```
cargo add cortex-m cortex-m-rt cortex-m-semihosting panic-halt
```

Add crate with features
```
cargo add stm32f4xx-hal --features stm32f469
```