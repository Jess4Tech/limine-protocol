use core::ptr::NonNull;

use crate::responses::SMPResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the bootloader bootstrap the secondary processors
    pub struct SMPRequest: [0x95a67b819a1b857e, 0xa0b61b723b6a73e0] {
        /// Response pointer
        pub response: Option<NonNull<SMPResponse>>,
        /// Flags for the bootloader
        /// `Bit 0` - Enable X2APIC if available
        pub flags: u64,
    }
}
