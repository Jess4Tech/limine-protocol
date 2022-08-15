use core::ptr::NonNull;

use crate::responses::EfiSystemTableResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the address of the EFI System Table
    pub struct EfiSystemTableRequest: [0x5ceba5163eaaf6d6, 0x0a6981610cf65fcc] {
        /// Response pointer
        pub response: Option<NonNull<EfiSystemTableResponse>>,
    }
}

impl EfiSystemTableRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&EfiSystemTableResponse> {
        Some(self.response?.as_ref())
    }
}
