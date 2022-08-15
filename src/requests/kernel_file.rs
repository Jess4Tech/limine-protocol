use core::ptr::NonNull;

use crate::responses::KernelFileResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request the file the Kernel was loaded from
    pub struct KernelFileRequest: [0xad97_e90e_83f1_ed67, 0x31eb_5d1c_5ff2_3b69] {
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
    #[must_use]
    pub unsafe fn get_response(&self) -> Option<&KernelFileResponse> {
        Some(self.response?.as_ref())
    }
}
