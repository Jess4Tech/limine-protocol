use core::ptr::NonNull;

use crate::responses::EntryPointResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the bootloader use the specified entry point instead of the default one
    pub struct EntryPointRequest: [0x13d86c035a1cd3e1, 0x2b0caa89d8f3026a] {
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
    pub unsafe fn get_response(&self) -> Option<&EntryPointResponse> {
        Some(self.response?.as_ref())
    }
}
