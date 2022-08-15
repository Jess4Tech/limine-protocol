use core::ptr::NonNull;

use crate::responses::SMPResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the bootloader bootstrap the secondary processors
    pub struct SMPRequest: [0x95a6_7b81_9a1b_857e, 0xa0b6_1b72_3b6a_73e0] {
        /// Response pointer
        pub response: Option<NonNull<SMPResponse>>,
        /// Flags for the bootloader
        /// `Bit 0` - Enable X2APIC if available
        pub flags: u64,
    }
}

impl SMPRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    #[must_use]
    pub unsafe fn get_response(&self) -> Option<&SMPResponse> {
        Some(self.response?.as_ref())
    }
}
