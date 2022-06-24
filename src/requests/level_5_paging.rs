use core::ptr::NonNull;

use crate::responses::Level5PagingResponse;

limine_request! {
    #[repr(C)]
    #[derive(Debug)]
    /// Request Level 5 paging be enabled
    pub struct Level5PagingRequest: [0x94469551da9b3192, 0xebe5e86db7382888] {
        /// Response pointer
        pub response: Option<NonNull<Level5PagingResponse>>,
    }
}
