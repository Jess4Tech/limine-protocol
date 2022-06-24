#[repr(C)]
#[derive(Debug)]
/// Response to [StackSizeRequest]
pub struct StackSizeResponse {
    /// The response revision number
    pub revision: u64,
}
