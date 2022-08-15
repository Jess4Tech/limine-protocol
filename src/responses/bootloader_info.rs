use core::ffi::CStr;

#[repr(C)]
#[derive(Debug)]
/// Response to [`InfoRequest`]
pub struct BootloaderInfoResponse<'a> {
    /// The response revision number
    pub revision: u64,
    /// A null-terminated string of the bootloader's name
    pub name: &'a CStr,
    /// A null-terminated string of the bootloader's version
    pub version: &'a CStr,
}
