use core::ptr::NonNull;

use crate::responses::MemoryMapResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request for receiving the Memory Map from the bootloader
    pub struct MemoryMapRequest: [0x67cf3d9d378a806f, 0xe304acdfc50c3c62] {
        /// Response pointer
        pub response: Option<NonNull<MemoryMapResponse>>,
    }
}

impl MemoryMapRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&MemoryMapResponse> {
        Some(self.response?.as_ref())
    }
}
