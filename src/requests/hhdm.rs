use core::ptr::NonNull;

use crate::responses::HHDMResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request Higher Half Direct Mapping be enabled
    pub struct HHDMRequest: [0x48dcf1cb8ad2b852, 0x63984e959a98244b] {
        /// Response pointer
        pub response: Option<NonNull<HHDMResponse>>,
    }
}
