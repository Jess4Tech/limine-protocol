use core::ptr::NonNull;

use crate::responses::RSDPResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the address of the RSDP
    pub struct RSDPRequest: [0xc5e77b6b397e7b43, 0x27637845accdcf3c] {
        /// Response pointer
        pub response: Option<NonNull<RSDPResponse>>,
    }
}
