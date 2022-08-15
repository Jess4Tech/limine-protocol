#[repr(C)]
#[derive(Debug)]
/// Response to [`SMBIOSRequest`]
pub struct SMBIOSResponse {
    /// The response revision number
    pub revision: u64,
    /// Address of the 32-bit SMBIOS entry point, null if not present
    pub entry_32: Option<*const u8>,
    /// Address of the 64-bit SMBIOS entry point, null if not present
    pub entry_64: Option<*const u8>,
}
