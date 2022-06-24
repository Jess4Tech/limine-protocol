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
