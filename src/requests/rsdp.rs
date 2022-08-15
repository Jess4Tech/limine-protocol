use core::ptr::NonNull;

use crate::responses::RSDPResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the address of the RSDP
    pub struct RSDPRequest: [0xc5e77b6b397e7b43, 0x27637845accdcf3c] {
        /// Response pointer
        pub response: Option<NonNull<RSDPResponse>>,
    }
}

impl RSDPRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&RSDPResponse> {
        Some(self.response?.as_ref())
    }
}
