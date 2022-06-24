#[repr(C)]
#[derive(Debug)]
/// Response to [RSDPRequest]
pub struct RSDPResponse {
    /// The response revision number
    pub revision: u64,
    /// Address of the RSDP table
    pub address: Option<*const u8>,
}
