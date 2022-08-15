use core::ptr::NonNull;

use crate::responses::BootTimeResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the time on boot
    pub struct BootTimeRequest: [0x502746e184c088aa, 0xfbc5ec83e6327893] {
        /// Response pointer
        pub response: Option<NonNull<BootTimeResponse>>,
    }
}

impl BootTimeRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&BootTimeResponse> {
        Some(self.response?.as_ref())
    }
}
