use crate::structures::memory_map_entry::MemoryMapEntry;

#[repr(C)]
#[derive(Debug)]
/// Response to [MemoryMapRequest]
pub struct MemoryMapResponse {
    /// The response revision number
    pub revision: u64,
    /// The amount of entries in the memory map
    pub entry_count: u64,
    /// An array of [MemoryMapEntry] pointers
    pub entries: *const *const MemoryMapEntry,
}

impl MemoryMapResponse {
    /// Get the memory map entry slice
    ///
    /// # Safety
    /// The pointer must point to a valid array of [MemoryMapEntry]
    pub unsafe fn get_memory_map(&self) -> Option<&[&MemoryMapEntry]> {
        if self.entries.is_null() {
            return None;
        }
        Some(core::slice::from_raw_parts(
            self.entries as *const &MemoryMapEntry,
            self.entry_count as usize,
        ))
    }
}
