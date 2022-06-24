#[repr(C)]
#[derive(Debug)]
/// Response to [BootTimeRequest]
pub struct BootTimeResponse {
    /// The response revision number
    pub revision: u64,
    /// The UNIX time on boot
    pub boot_time: i64,
}
