# TedOS

Experimental toy OS written in Rust

## Installing RiscV assembler and linker on macOS

`brew install riscv-tools`

## Creating a QEMU disck image

`qemu-img create tedos-fs.img 32M`

## Running in QEMU

`./build.sh run`

To exit QEMU, press `ctrl + a` followed by `x`

## Learning resources
  - https://osblog.stephenmarz.com/ch1.html
  - https://github.com/mit-pdos/xv6-riscv/tree/riscv
  - https://www.meyerzinn.tech/posts/2023/03/05/running-rust-code-on-risc-v-in-qemu/
  - https://os.phil-opp.com

## TODO

* Improve Mutex to handle poisoned mutex: https://whenderson.dev/blog/rust-mutexes/
* Get allocator working
* Alloc a struct per hart
  - Should contain a counter of disabled interrupts (meaning, everytime interrupts are disabled, increment. When reenabled, decrement)
* Mutex should only reenable interrupts if CPU interrupt disable counter goes down to 0
* Use mutex for printbuf
* Use alloc for printbuf
* Init UART from kernel_main, not printbuf
* Init PLIC
* Panic should interrupt other harts
* Once hart 0 finishes boot, interrupt other harts
* Get booting with UBoot and OpenSBI
