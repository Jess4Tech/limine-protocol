#[repr(C)]
#[derive(Debug)]
/// Response to [KernelAddressRequest]
pub struct KernelAddressResponse {
    /// The response revision number
    pub revision: u64,
    /// The physical base of the kernel
    pub physical_base: u64,
    /// The virtual base of the kernel
    pub virtual_base: u64,
}
