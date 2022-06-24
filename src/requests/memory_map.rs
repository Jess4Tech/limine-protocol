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
