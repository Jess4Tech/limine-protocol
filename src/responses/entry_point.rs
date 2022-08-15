#[repr(C)]
#[derive(Debug)]
/// Response to [`EntryPointRequest`]
pub struct EntryPointResponse {
    /// The response revision number
    pub revision: u64,
}
