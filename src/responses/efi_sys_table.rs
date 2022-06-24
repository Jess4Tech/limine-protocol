#[repr(C)]
#[derive(Debug)]
/// Response to [EfiSystemTableRequest]
pub struct EfiSystemTableResponse {
    /// The response revision number
    pub revision: u64,
    /// Address of the EFI System Table
    pub address: Option<*const u8>,
}
