#[repr(C)]
#[derive(Debug)]
/// Response to [`Level5PagingRequest`]
pub struct Level5PagingResponse {
    /// The response revision number
    pub revision: u64,
}
