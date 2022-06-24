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
