use core::ptr::NonNull;

use crate::responses::ModuleResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the loaded modules
    pub struct ModuleRequest: [0x3e7e279702be32af, 0xca1c4f3bd1280cee] {
        /// Response pointer
        pub response: Option<NonNull<ModuleResponse<'static>>>,
    }
}
