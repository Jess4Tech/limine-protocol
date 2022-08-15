use core::ptr::NonNull;

use crate::responses::HHDMResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request Higher Half Direct Mapping be enabled
    pub struct HHDMRequest: [0x48dcf1cb8ad2b852, 0x63984e959a98244b] {
        /// Response pointer
        pub response: Option<NonNull<HHDMResponse>>,
    }
}

impl HHDMRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&HHDMResponse> {
        Some(self.response?.as_ref())
    }
}
