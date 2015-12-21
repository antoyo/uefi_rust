#![feature(intrinsics)]

pub type EFI_HANDLE = *const ();

struct EFI_TABLE_HEADER {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    CRC32: u32,
    Reserved: u32,
}

pub struct EFI_SYSTEM_TABLE {
    Hdr: EFI_TABLE_HEADER,
    FirmwareVendor: *const u16,
    FirmwareRevision: u32,
    ConsoleInHandle: EFI_HANDLE,
    ConIn: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle: EFI_HANDLE,
    ConOut: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    // TODO
}

struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: EFI_TEXT_RESET,
    OutputString: EFI_TEXT_STRING,
    // TODO
}

type EFI_TEXT_RESET = *const ();

type EFI_TEXT_STRING = extern fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *const u16);

#[no_mangle]
pub extern "win64" fn efi_main(_ImageHandle: EFI_HANDLE, SystemTable: *const EFI_SYSTEM_TABLE) -> i32 {
    unsafe {
        let ref SystemTable = *SystemTable;
        let vendor = SystemTable.FirmwareVendor;
        let conout = SystemTable.ConOut;
        let output = (*conout).OutputString;

        let hello = [
            'H' as u16,
            'e' as u16,
            'l' as u16,
            'l' as u16,
            'o' as u16,
            ',' as u16,
            ' ' as u16,
            'W' as u16,
            'o' as u16,
            'r' as u16,
            'l' as u16,
            'd' as u16,
            '\r' as u16,
            '\n' as u16,
            0u16,
        ];

        let (hello_ptr, _) = buf_ptr(&hello);

        output(conout, hello_ptr);
    }

    loop {}
}

fn buf_ptr<T>(buf: &[T]) -> (*const T, u32) {
    unsafe {
        transmute(buf)
    }
}

extern "rust-intrinsic" {
    pub fn transmute<T, U>(val: T) -> U;
}
