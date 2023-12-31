#!/bin/sh

TARGET_ARCH=riscv64
TARGET_MACHINE=rv64qemu

if [[ $2 = "release" ]]; then
	BUILD_PROFILE=release
elif [[ $2 = "debug" || $2 = "" ]]; then
	BUILD_PROFILE=debug
else
    echo "Invalid build profile: $2"
    exit 1
fi

if [[ $TARGET_ARCH == "riscv64" ]]; then
    ASM_SRCS="./src/platform/riscv/boot.asm ./src/platform/riscv/timer_interrupt_entry.asm"

    CARGO_BUILD_TARGET=riscv64gc-unknown-none-elf
    RS_OUT_DIR=./target/riscv64gc-unknown-none-elf/$BUILD_PROFILE

    AS=riscv64-unknown-elf-as
    AS_FLAGS="-Wall -march=rv64gc -mabi=lp64d"
    AS_OUT=$RS_OUT_DIR/boot.o

    LD=riscv64-unknown-elf-ld
    LD_FLAGS="-static -nostdlib --no-warn-rwx-segment"
    LD_OUT=tedos-kernel.elf

    KERNEL_LIB=tedoskernel

    if [[ $TARGET_MACHINE == "rv64qemu" ]]; then
        LD_SCRIPT=-T./src/platform/riscv/qemu/virt.ld

        QEMU=qemu-system-riscv64
        QEMU_DISK=tedos-fs.img

        # Before changing these, make sure to update the linker script,
        # boot entry, and constants for QEMU in the Rust code to match
        # the new parameters.
        QEMU_HART_COUNT=4
        QEMU_MEM=256M
    fi
fi

mkdir -p .cargo
echo "# Autogenerated by build.sh" > .cargo/config.toml
echo "[build]" >> .cargo/config.toml
echo "target = \"$CARGO_BUILD_TARGET\"" >> .cargo/config.toml
echo "rustflags = [\"--cfg\", \"target_machine=\\\"$TARGET_MACHINE\\\"\"]" >> .cargo/config.toml

function build {
	if [[ $BUILD_PROFILE = "debug" ]]; then
        (PS4="\000" set -x;
	     cargo build &&
	         $AS $AS_FLAGS $ASM_SRCS -o $AS_OUT 1> /dev/null &&
             $LD $LD_FLAGS $LD_SCRIPT $AS_OUT -o $LD_OUT -L$RS_OUT_DIR -l$KERNEL_LIB
        ) || exit 1
	elif [[ $BUILD_PROFILE = "release" ]]; then
        (PS4="\000" set -x;
	     cargo build --release &&
             $AS $AS_FLAGS $ASM_SRCS -o $AS_OUT 1> /dev/null &&
             $LD $LD_FLAGS $LD_SCRIPT $AS_OUT -o $LD_OUT -L$RS_OUT_DIR -l$KERNEL_LIB
        ) || exit 1
    fi
}

if [[ $1 = "build" ]]; then
	build
elif [[ $1 = "run" ]]; then
    build &&
    $QEMU -machine virt -cpu rv64 \
        -smp $QEMU_HART_COUNT -m $QEMU_MEM \
        -nographic -serial mon:stdio \
        -bios none \
        -kernel $LD_OUT \
        -drive file=$QEMU_DISK,if=none,format=raw,id=dsk0 \
        -device virtio-blk-device,drive=dsk0,bus=virtio-mmio-bus.0
elif [[ $1 = "check" ]]; then
    cargo check
elif [[ $1 = "clippy" ]]; then
    cargo clippy
elif [[ $1 = "clean" ]]; then
    cargo clean
    rm -f $LD_OUT
else
    echo "Usage: ./$(basename $0) <build|run|check|clippy|clean> <optional:debug|release>"
    exit 1
fi
