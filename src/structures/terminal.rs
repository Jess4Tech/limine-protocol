use super::framebuffer::Framebuffer;

/// Type for terminal `Write` function
/// In order for the terminal to remain functioning, the GDT must at least have the default values
///
/// Special length values do the following things
/// * `TerminalSize` - `-1`
///     * This will **write** a single u64 to the pointer
/// * `TerminalSave` - `-2`
///     * This will save the terminal context to the pointer,5 of allocation size `TerminalSize`
/// * `TerminalRestore` - `-3`
///     * This will restore the terminal context from the pointer, of allocation size `TerminalSize`
/// * `TerminalFullRefresh` - `-4`
///     * This will fully repaint the framebuffer, the pointer is unused
pub type TerminalWriteFn = extern "C" fn(*mut Terminal, *const u8, u64);

/// Type for terminal callback
/// The first argument is `Type`, which corresponds to `CallbackType`[CallbackType]
/// * `DecSequence`
///     * `ValueCount` - Number of values in the array `Values`
///     * `Values` - Array of values in the DEC Sequence
///     * `Final` - The final character in the DEC Sequence
/// * `Bell`
///     * `unused1`
///     * `unused2`
///     * `unused3`
/// * `DECPrivateID`
///     * `unused1`
///     * `unused2`
///     * `unused3`
/// * `StatusReport`
///     * `unused1`
///     * `unused2`
///     * `unused3`
/// * `CursorPositionReport`
///     * `X` - The X position at the time the report was requested
///     * `Y` - The Y position at the time the report was requested
///     * `unused3`
/// * `KeyboardLEDUpdate`
///     * `LedState` - What to set (possible values below)
///         * `0` - Clear all LEDs
///         * `1` - Set Scroll Lock
///         * `2` - Set Num Lock
///         * `3` - Set Caps Lock LED
///     * `unused2`
///     * `unused3`
/// * `SwitchSequence`
///     * `ValueCount` - Number of values in the array `Values`
///     * `Values` - Array of values in the Switch Sequence
///     * `Final` - The final character in the Switch Sequence
/// * `LinuxEscapeSequence`
///     * `ValueCount` - Number of values in the array `Values`
///     * `Values` - Array of values in the Sequence
pub type TerminalCallback = extern "C" fn(*mut Terminal, u64, u64, u64, u64);

#[repr(u64)]
#[derive(Debug, PartialEq, Eq)]
/// Types of terminal callbacks
pub enum CallbackType {
    /// A DEC Private Mode Sequence has been encountered
    DECSequence = 10,
    /// A bell event has occurred
    Bell = 20,
    /// The kernel must respond to a DEC Private ID request
    DECPrivateID = 30,
    /// The kernel must respond to an ECMA-48 status report request
    StatusReport = 40,
    /// The kernel must respond to an ECMA-48 cursor position report request
    CursorPositionReport = 50,
    /// The kernel must respond to a keyboard LED change request
    KeyboardLEDUpdate = 60,
    /// An ECMA-48 Switch sequence has been encountered that the terminal cannot handle alone
    SwitchSequence = 70,
    /// A private Linux escape sequence has been encountered that the terminal cannot handle alone
    LinuxEscapeSequence = 80,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
/// Terminal structure
pub struct Terminal {
    /// How many columns are in the terminal
    pub columns: u32,
    /// How many rows are in the terminal
    pub rows: u32,
    /// The terminal's framebuffer
    pub framebuffer: *mut Framebuffer,
}
