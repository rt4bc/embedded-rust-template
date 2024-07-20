# Setup Basic Dependency

## rust setup
For bandwidth and disk usage concerns the default installation only supports native compilation. To add cross compilation support for the ARM Cortex-M architectures choose one of the following compilation targets. For the STM32F3DISCOVERY board used for the examples in this book, use the thumbv7em-none-eabihf target.

Cortex-M0, M0+, and M1 (ARMv6-M architecture):

rustup target add thumbv6m-none-eabi
Cortex-M3 (ARMv7-M architecture):

rustup target add thumbv7m-none-eabi
Cortex-M4 and M7 without hardware floating point (ARMv7E-M architecture):

rustup target add thumbv7em-none-eabi
Cortex-M4F and M7F with hardware floating point (ARMv7E-M architecture):

rustup target add thumbv7em-none-eabihf
Cortex-M23 (ARMv8-M architecture):

rustup target add thumbv8m.base-none-eabi
Cortex-M33 and M35P (ARMv8-M architecture):

rustup target add thumbv8m.main-none-eabi
Cortex-M33F and M35PF with hardware floating point (ARMv8-M architecture):
rustup target add thumbv8m.main-none-eabihf


cargo-generate
We'll use this later to generate a project from a template.

cargo install cargo-generate
Note: on some Linux distros (e.g. Ubuntu) you may need to install the packages libssl-dev and pkg-config prior to installing cargo-generate.


## Cortex-M
* 添加基础crate
```shell
cargo add cortex-m cortex-m-rt panic-halt
```

* 添加HAL Crate

```shell
cargo add stm32f4xx-hal --features stm32f469
```
或者在Cargo.toml里直接添加
```toml
[dependencies.stm32f4xx-hal]
version = "0.15.0"
features = ["stm32f411"]
```

* 创建.cargo/config.toml文件,设置目标和链接器:
```toml
[target.thumbv7em-none-eabihf]
rustflags = [
  "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv7em-none-eabihf"
```