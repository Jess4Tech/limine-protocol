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
