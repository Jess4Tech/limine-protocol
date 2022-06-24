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
