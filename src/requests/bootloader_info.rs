use core::ptr::NonNull;

use crate::responses::BootloaderInfoResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Get information regarding the bootloader
    pub struct BootloaderInfoRequest: [0xf550_38d8_e2a1_202f, 0x2794_26fc_f5f5_9740] {
        /// Response pointer
        pub response: Option<NonNull<BootloaderInfoResponse<'static>>>,
    }
}

impl BootloaderInfoRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    #[must_use]
    pub unsafe fn get_response(&self) -> Option<&BootloaderInfoResponse> {
        Some(self.response?.as_ref())
    }
}
