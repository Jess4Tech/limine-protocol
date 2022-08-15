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

impl KernelAddressRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&KernelAddressResponse> {
        Some(self.response?.as_ref())
    }
}
