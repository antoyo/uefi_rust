use std::env::consts::ARCH;
use std::process::Command;

fn main() {
    let mut args = vec!["-cpu", "qemu64", "-bios", "/usr/share/ovmf/ovmf_x64.bin", "-drive", "file=target/debug/disk.img,if=ide,format=raw"];
    if cfg!(feature = "debug") {
        args.append(&mut vec!["-d", "int", "-no-reboot"]);
    }
    Command::new(format!("qemu-system-{}", ARCH))
        .args(&args)
        .status().unwrap();
}
