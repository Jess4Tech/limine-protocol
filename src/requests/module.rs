use core::ptr::NonNull;

use crate::responses::ModuleResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the loaded modules
    pub struct ModuleRequest: [0x3e7e_2797_02be_32af, 0xca1c_4f3b_d128_0cee] {
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
    #[must_use]
    pub unsafe fn get_response(&self) -> Option<&ModuleResponse> {
        Some(self.response?.as_ref())
    }
}
