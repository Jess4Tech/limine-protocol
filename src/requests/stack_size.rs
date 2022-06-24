use core::ptr::NonNull;

use crate::responses::StackSizeResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Specify how much stack you desire the bootloader to give you
    pub struct StackSizeRequest: [0x224ef0460a8e8926, 0xe1cb0fc25f46ea3d] {
        /// Response pointer
        pub response: Option<NonNull<StackSizeResponse>>,
        /// The amount of stack to request
        pub stack_size: u64,
    }
}
