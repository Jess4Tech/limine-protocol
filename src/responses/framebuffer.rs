use crate::structures::framebuffer::Framebuffer;

#[repr(C)]
#[derive(Debug)]
/// Response to [FramebufferRequest]
pub struct FramebufferResponse {
    /// The response revision number
    pub revision: u64,
    /// The number of [Framebuffer]s in `framebuffers`
    pub framebuffer_count: u64,
    /// A pointer to an array of [Framebuffer] pointers
    pub framebuffers: *mut *mut Framebuffer,
}

impl FramebufferResponse {
    /// Get the framebuffer slice
    /// # Safety
    /// The pointer must point to a valid array of [Framebuffer]s
    pub unsafe fn get_framebuffers(&self) -> Option<&[&Framebuffer]> {
        if self.framebuffers.is_null() {
            return None;
        }
        Some(core::slice::from_raw_parts(
            self.framebuffers as *const &Framebuffer,
            self.framebuffer_count as usize,
        ))
    }
}
