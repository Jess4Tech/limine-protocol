use core::ptr::NonNull;

use crate::responses::SMBIOSResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the SMBIOS entry point
    pub struct SMBIOSRequest: [0x9e9046f11e095391, 0xaa4a520fefbde5ee] {
        /// Response pointer
        pub response: Option<NonNull<SMBIOSResponse>>,
    }
}

impl SMBIOSRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&SMBIOSResponse> {
        Some(self.response?.as_ref())
    }
}
