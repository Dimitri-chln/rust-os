/// Choose whether to start the UEFI or BIOS image
const UEFI: bool = false;

fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let mut cmd = std::process::Command::new("qemu-system-x86_64");

    cmd.arg("-no-shutdown").arg("-no-reboot");

    if UEFI {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());

        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
