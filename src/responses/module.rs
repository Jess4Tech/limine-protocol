use crate::structures::file::File;

#[repr(C)]
#[derive(Debug)]
/// Response to [`ModuleRequest`]
pub struct ModuleResponse<'a> {
    /// The response revision number
    pub revision: u64,
    /// The number of modules
    pub module_count: u64,
    /// An array of [File] pointers
    pub modules: *const *const File<'a>,
}
