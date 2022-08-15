use core::ptr::NonNull;

use crate::responses::BootloaderInfoResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Get information regarding the bootloader
    pub struct BootloaderInfoRequest: [0xf55038d8e2a1202f, 0x279426fcf5f59740] {
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
    pub unsafe fn get_response(&self) -> Option<&BootloaderInfoResponse> {
        Some(self.response?.as_ref())
    }
}
