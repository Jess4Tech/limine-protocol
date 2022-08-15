use core::ptr::NonNull;

use crate::responses::ModuleResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the loaded modules
    pub struct ModuleRequest: [0x3e7e279702be32af, 0xca1c4f3bd1280cee] {
        /// Response pointer
        pub response: Option<NonNull<ModuleResponse<'static>>>,
    }
}

impl ModuleRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&ModuleResponse> {
        Some(self.response?.as_ref())
    }
}
