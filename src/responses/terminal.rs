use crate::structures::terminal::{Terminal, TerminalWriteFn};

#[repr(C)]
#[derive(Debug)]
/// Response to [`TerminalRequest`]
pub struct TerminalResponse {
    /// The response revision number
    pub revision: u64,
    /// The number of [Terminal]s in `terminals`
    pub terminal_count: u64,
    /// A pointer to an array of [Terminal] pointers
    pub terminals: *const *const Terminal,
    /// The terminal write function
    ///
    /// # Important
    /// It must be noted that this is the physical address of the write function
    pub write: TerminalWriteFn,
}

impl TerminalResponse {
    /// Get the terminal slice
    ///
    /// # Safety
    /// The pointer must point to a valid array of [Terminal]s
    #[must_use]
    pub unsafe fn get_terminals(&self) -> Option<&[&Terminal]> {
        if self.terminals.is_null() {
            return None;
        }
        Some(core::slice::from_raw_parts(
            self.terminals.cast::<&Terminal>(),
            self.terminal_count.try_into().ok()?,
        ))
    }

    /// Get the terminal slice
    ///
    /// # Safety
    /// The pointer must point to a valid array of [Terminal]s.
    /// Additionally, you must ensure that this is called *nowhere* else, otherwise
    /// very, very bad things may occur due to read and write tearing
    #[must_use]
    pub unsafe fn get_terminals_mut(&self) -> Option<&mut [&mut Terminal]> {
        if self.terminals.is_null() {
            return None;
        }
        Some(core::slice::from_raw_parts_mut(
            self.terminals as *mut &mut Terminal,
            self.terminal_count.try_into().ok()?,
        ))
    }
}
