use core::ptr::NonNull;

use crate::responses::TerminalResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request a terminal
    pub struct TerminalRequest: [0x0785a0aea5d0750f, 0x1c1936fee0d6cf6e] {
        /// Response pointer
        pub response: Option<NonNull<TerminalResponse>>,
    }
}
