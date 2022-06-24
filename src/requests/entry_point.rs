use core::ptr::NonNull;

use crate::responses::EntryPointResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the bootloader use the specified entry point instead of the default one
    pub struct EntryPointRequest: [0x13d86c035a1cd3e1, 0x2b0caa89d8f3026a] {
        /// Response pointer
        pub response: Option<NonNull<EntryPointResponse>>,
        /// Function to the entry point
        pub entry: Option<*mut ()>,
    }
}
