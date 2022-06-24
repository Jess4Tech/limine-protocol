use core::ptr::NonNull;

use crate::responses::BootTimeResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the time on boot
    pub struct BootTimeRequest: [0x502746e184c088aa, 0xfbc5ec83e6327893] {
        /// Response pointer
        pub response: Option<NonNull<BootTimeResponse>>,
    }
}
