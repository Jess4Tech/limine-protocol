use core::ptr::NonNull;

use crate::responses::Level5PagingResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request Level 5 paging be enabled
    pub struct Level5PagingRequest: [0x94469551da9b3192, 0xebe5e86db7382888] {
        /// Response pointer
        pub response: Option<NonNull<Level5PagingResponse>>,
    }
}

impl Level5PagingRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&Level5PagingResponse> {
        Some(self.response?.as_ref())
    }
}
