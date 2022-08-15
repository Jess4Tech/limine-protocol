use core::ptr::NonNull;

use crate::responses::FramebufferResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request a framebuffer
    pub struct FramebufferRequest: [0xcbfe81d7dd2d1977, 0x063150319ebc9b71] {
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
    pub unsafe fn get_response(&self) -> Option<&FramebufferResponse> {
        Some(self.response?.as_ref())
    }
}
