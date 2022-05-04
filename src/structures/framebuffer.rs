#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
/// Framebuffer structure
pub struct Framebuffer {
    /// The address of the framebuffer
    pub address: *mut u8,
    /// The width of the framebuffer
    pub width: u16,
    /// The height of the framebuffer
    pub height: u16,
    /// The pitch of the framebuffer
    pub pitch: u16,
    /// How many bits are present per pixel
    pub bpp: u16,
    /// The memory model of the framebuffer
    pub memory_model: u8,
    /// The red mask size
    pub red_mask_size: u8,
    /// The red mask shift amount
    pub red_mask_shift: u8,
    /// The green mask size
    pub green_mask_size: u8,
    /// The green mask shift amount
    pub green_mask_shift: u8,
    /// The blue mask size
    pub blue_mask_size: u8,
    /// The blue mask shift amount
    pub blue_mask_shift: u8,
    /// This is unused, you shouldn't even see this.
    unused: u8,
    /// The size of the EDID
    pub edid_size: u64,
    /// A pointer to the EDID
    pub edid: *mut u8,
}

#[cfg(test)]
impl Framebuffer {
    pub fn new_empty() -> Self {
        use core::ptr::null_mut;
        Self {
            address: null_mut(),
            width: 100,
            height: 100,
            pitch: 3,
            bpp: 3,
            memory_model: 0,
            red_mask_size: 0,
            red_mask_shift: 0,
            green_mask_size: 0,
            green_mask_shift: 0,
            blue_mask_size: 0,
            blue_mask_shift: 0,
            unused: 0,
            edid_size: 0,
            edid: null_mut(),
        }
    }
}
