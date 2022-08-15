use core::ptr::NonNull;

use crate::responses::FramebufferResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request a framebuffer
    pub struct FramebufferRequest: [0xcbfe_81d7_dd2d_1977, 0x0631_5031_9ebc_9b71] {
        /// Response pointer
        pub response: Option<NonNull<FramebufferResponse>>,
    }
}

impl FramebufferRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    #[must_use]
    pub unsafe fn get_response(&self) -> Option<&FramebufferResponse> {
        Some(self.response?.as_ref())
    }
}
