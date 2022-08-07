#[repr(C)]
#[derive(Debug)]
/// Response to [HHDMRequest]
pub struct HHDMResponse {
    /// The response revision number
    pub revision: u64,
    /// The offset of the HHDM
    pub offset: *mut (),
}
