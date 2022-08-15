use crate::structures::file::File;

#[repr(C)]
#[derive(Debug)]
/// Response to [`KernelFileRequest`]
pub struct KernelFileResponse<'a> {
    /// The response revision number
    pub revision: u64,
    /// Pointer to the kernel file structure
    pub kernel_file: *const File<'a>,
}
