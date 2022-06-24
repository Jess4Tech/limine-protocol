use core::ptr::NonNull;

use crate::responses::KernelAddressResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the physical and virtual address of the Kernel
    pub struct KernelAddressRequest: [0x71ba76863cc55f63, 0xb2644a48c516a487] {
        /// Response pointer
        pub response: Option<NonNull<KernelAddressResponse>>,
    }
}
