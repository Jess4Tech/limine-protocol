use core::ptr::NonNull;

use crate::responses::EntryPointResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the bootloader use the specified entry point instead of the default one
    pub struct EntryPointRequest: [0x13d8_6c03_5a1c_d3e1, 0x2b0c_aa89_d8f3_026a] {
        /// Response pointer
        pub response: Option<NonNull<EntryPointResponse>>,
        /// Function to the entry point
        pub entry: Option<*mut ()>,
    }
}

impl EntryPointRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    #[must_use]
    pub unsafe fn get_response(&self) -> Option<&EntryPointResponse> {
        Some(self.response?.as_ref())
    }
}
