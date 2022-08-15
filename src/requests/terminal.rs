use core::ptr::NonNull;

use crate::responses::TerminalResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request a terminal
    pub struct TerminalRequest: [0x0785_a0ae_a5d0_750f, 0x1c19_36fe_e0d6_cf6e] {
        /// Response pointer
        pub response: Option<NonNull<TerminalResponse>>,
    }
}

impl TerminalRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    #[must_use]
    pub unsafe fn get_response(&self) -> Option<&TerminalResponse> {
        Some(self.response?.as_ref())
    }
}
