use std::fs::create_dir;
use std::process::Command;

fn main() {
    build();
    link();
    iso();
}

fn build() {
    Command::new("cargo")
        .current_dir("uefi_app")
        .args(&["rustc", "--", "--emit", "obj"])
        .status().unwrap();
}

fn iso() {
    let disk_file = "target/debug/disk.img";
    let disk_dir = "target/debug/disk";
    let efi_boot = disk_dir.to_owned() + "/efi/boot";
    let copy_dest = efi_boot.clone() + "/bootx64.efi";

    let dd_of = "of=".to_owned() + disk_file;
    Command::new("dd")
        .args(&["if=/dev/zero", &dd_of, "bs=512", "count=93750"])
        .status().unwrap();

    Command::new("parted")
        .args(&[disk_file, "-s", "-a", "minimal", "mklabel", "gpt"])
        .status().unwrap();

    Command::new("parted")
        .args(&[disk_file, "-s", "-a", "minimal", "mkpart", "EFI", "FAT16", "2048s", "93716s"])
        .status().unwrap();

    Command::new("parted")
        .args(&[disk_file, "-s", "-a", "minimal", "toggle", "1", "boot"])
        .status().unwrap();

    Command::new("sudo")
        .args(&["losetup", "--offset", "1048576", "--sizelimit", "46934528", "/dev/loop0", disk_file])
        .status().unwrap();

    let _ = create_dir(disk_dir);

    Command::new("sudo")
        .args(&["mkdosfs", "-F", "32", "/dev/loop0"])
        .status().unwrap();

    Command::new("sudo")
        .args(&["mount", "/dev/loop0", disk_dir])
        .status().unwrap();

    Command::new("sudo")
        .args(&["mkdir", "-p", &efi_boot])
        .status().unwrap();

    Command::new("sudo")
        .args(&["cp", "target/debug/boot.efi", &copy_dest])
        .status().unwrap();

    Command::new("sudo")
        .args(&["umount", disk_dir])
        .status().unwrap();

    Command::new("sudo")
        .args(&["losetup", "-d", "/dev/loop0"])
        .status().unwrap();

    Command::new("sudo")
        .args(&["rm", "-R", disk_dir])
        .status().unwrap();
}

fn link() {
    Command::new("x86_64-efi-pe-ld")
        .args(&["--oformat", "pei-x86-64", "--subsystem", "10", "-pie", "-e", "efi_main", "uefi_app/target/debug/uefi_app.o", "-o", "target/debug/boot.efi"]).output().unwrap();
}
