use core::ptr::NonNull;

use crate::responses::HHDMResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request Higher Half Direct Mapping be enabled
    pub struct HHDMRequest: [0x48dc_f1cb_8ad2_b852, 0x6398_4e95_9a98_244b] {
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
    #[must_use]
    pub unsafe fn get_response(&self) -> Option<&HHDMResponse> {
        Some(self.response?.as_ref())
    }
}
