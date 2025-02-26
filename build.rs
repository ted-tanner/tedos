fn main() {
    println!("cargo::rustc-check-cfg=cfg(target_machine, values(\"rv64qemu\"))");
}
