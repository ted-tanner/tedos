# TedOS

Experimental toy OS written in Rust

## Installing RiscV assembler and linker on macOS

`brew install riscv-tools`

## Creating a QEMU disck image

`qemu-img create tedos-fs.img 32M`

## Learning resources
  - https://osblog.stephenmarz.com/ch1.html
  - https://github.com/mit-pdos/xv6-riscv/tree/riscv
  - https://www.meyerzinn.tech/posts/2023/03/05/running-rust-code-on-risc-v-in-qemu/
  - https://os.phil-opp.com

## TODO

* Create reusable lock code
  * Mutex
  * InitLock
