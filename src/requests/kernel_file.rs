use core::ptr::NonNull;

use crate::responses::KernelFileResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the file the Kernel was loaded from
    pub struct KernelFileRequest: [0xad97e90e83f1ed67, 0x31eb5d1c5ff23b69] {
        /// Response pointer
        pub response: Option<NonNull<KernelFileResponse<'static>>>,
    }
}

impl KernelFileRequest {
    /// Get the response as a reference, if it's present.
    ///
    /// # Safety
    /// The backing memory must not have been invalidated by the kernel,
    /// either by writing to the physical memory, changing where it's mapped, or
    /// unmapping it.
    pub unsafe fn get_response(&self) -> Option<&KernelFileResponse> {
        Some(self.response?.as_ref())
    }
}
