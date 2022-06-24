#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
/// UUID
pub struct UUID {
    /// First part
    pub a: u32,
    /// Second part
    pub b: u16,
    /// Third part
    pub c: u16,
    /// Fourth part
    pub d: [u8; 8],
}
