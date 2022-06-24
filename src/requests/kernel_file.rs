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
