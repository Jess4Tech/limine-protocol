use core::ptr::NonNull;

use crate::responses::StackSizeResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Specify how much stack you desire the bootloader to give you
    pub struct StackSizeRequest: [0x224e_f046_0a8e_8926, 0xe1cb_0fc2_5f46_ea3d] {
        /// Response pointer
        pub response: Option<NonNull<StackSizeResponse>>,
        /// The amount of stack to request
        pub stack_size: u64,
    }
}

impl StackSizeRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    #[must_use]
    pub unsafe fn get_response(&self) -> Option<&StackSizeResponse> {
        Some(self.response?.as_ref())
    }
}
